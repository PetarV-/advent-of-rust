use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::cell::RefCell;

// necessary in order to enforce copy rather than move semantics!
#[derive(Clone)]
enum Gate {
    ConstVal(u16),
    Const(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16)
}

struct Node {
    orig: Gate, 
    val : u16,
    ideg: u16,
    adj : Vec<String>
}

struct Graph {
    names: HashMap<String, usize>,
    nodes: Vec<RefCell<Node>>
}

impl Node {
    fn new() -> Node {
        Node {
            orig: Gate::ConstVal(0), //dummy gate
            val : 0,
            ideg: 0,
            adj : Vec::new()
        }
    }
}

impl Graph {
    fn new() -> Graph {
        Graph {
            names: HashMap::new(),
            nodes: Vec::new()
        }
    }

    fn spawn_if_missing(&mut self, name: String) {
        if !self.names.contains_key(&name) {
            let index = self.nodes.len();
            self.nodes.push(RefCell::new(Node::new()));
            self.names.insert(name, index);
        }
    }

    fn spawn_or_parse_if_missing(&mut self, name: String) {
        if !self.names.contains_key(&name) {
            match name.parse() {
                Ok(num) => {
                    self.spawn_if_missing(name.clone());
                    let mut node = self.nodes[self.names[&name]].borrow_mut();
                    node.orig = Gate::ConstVal(num);
                } 
                Err(_)  => {
                    let index = self.nodes.len();
                    self.nodes.push(RefCell::new(Node::new()));
                    self.names.insert(name, index);
                }
            }
        }
    }

    fn add_edge(&mut self, from: String, to: String) {
        let mut node_a = self.nodes[self.names[&from]].borrow_mut();
        node_a.adj.push(to.clone());
        
    }

    fn remove_edge(&mut self, from: String, to: String) {
        let mut node_a = self.nodes[self.names[&from]].borrow_mut();
        let index = node_a.adj.iter().position(|x| *x == to).unwrap();
        node_a.adj.remove(index);
    }

    fn add_connection(&mut self, gate: Gate, output: String) {
        self.spawn_if_missing(output.clone());
        {
            let mut node = self.nodes[self.names[&output]].borrow_mut();
            node.orig = gate.clone();
        }
        match gate {
            Gate::ConstVal(_)    => { },
            Gate::Const(x)       => { 
                self.spawn_or_parse_if_missing(x.clone());
                self.add_edge(x.clone(), output.clone());
            },
            Gate::Not(x)         => {
                self.spawn_or_parse_if_missing(x.clone());
                self.add_edge(x.clone(), output.clone());
            },
            Gate::And(x, y)      => {
                self.spawn_or_parse_if_missing(x.clone());
                self.spawn_or_parse_if_missing(y.clone());
                self.add_edge(x.clone(), output.clone());
                self.add_edge(y.clone(), output.clone());
            },
            Gate::Or(x, y)       => {
                self.spawn_or_parse_if_missing(x.clone());
                self.spawn_or_parse_if_missing(y.clone());
                self.add_edge(x.clone(), output.clone());
                self.add_edge(y.clone(), output.clone());
            },
            Gate::LShift(x, _)   => {
                self.spawn_or_parse_if_missing(x.clone());
                self.add_edge(x.clone(), output.clone());
            },
            Gate::RShift(x, _)   => {
                self.spawn_or_parse_if_missing(x.clone());
                self.add_edge(x.clone(), output.clone());
            }
        }
    }

    fn cut(&mut self, name: String) {
        let old_gate;
        {
            let mut node = self.nodes[self.names[&name]].borrow_mut();
            old_gate = node.orig.clone();
            node.orig = Gate::ConstVal(0); // dummy gate
        }
        match old_gate {
            Gate::ConstVal(_)    => { },
            Gate::Const(x)       => { 
                self.remove_edge(x.clone(), name.clone());
            },
            Gate::Not(x)         => {
                self.remove_edge(x.clone(), name.clone());
            },
            Gate::And(x, y)      => {
                self.remove_edge(x.clone(), name.clone());
                self.remove_edge(y.clone(), name.clone());
            },
            Gate::Or(x, y)       => {
                self.remove_edge(x.clone(), name.clone());
                self.remove_edge(y.clone(), name.clone());
            },
            Gate::LShift(x, _)   => {
                self.remove_edge(x.clone(), name.clone());
            },
            Gate::RShift(x, _)   => {
                self.remove_edge(x.clone(), name.clone());
            }
        }
    }

    fn recompute_indegrees(&mut self) {
        for ref_node in self.nodes.iter() {
            ref_node.borrow_mut().ideg = 0;
        }

        for ref_node in self.nodes.iter() {
            let node = ref_node.borrow();
            let neighbours = node.adj.clone();
            for n_name in neighbours.iter() {
                let mut neighbour = self.nodes[self.names[n_name]].borrow_mut();
                neighbour.ideg += 1;
            }
        }
    }

    fn expand(&mut self, name: String, queue: &mut Vec<String>) -> u16 {
        let mut node = self.nodes[self.names[&name]].borrow_mut();
        node.val = match node.orig.clone() {
            Gate::ConstVal(val)  => val,
            Gate::Const(x)       => self.nodes[self.names[&x]].borrow().val,
            Gate::Not(x)         => !self.nodes[self.names[&x]].borrow().val,
            Gate::And(x, y)      => {
                self.nodes[self.names[&x]].borrow().val 
                & self.nodes[self.names[&y]].borrow().val
            },
            Gate::Or(x, y)       => {
                self.nodes[self.names[&x]].borrow().val 
                | self.nodes[self.names[&y]].borrow().val
            },
            Gate::LShift(x, val) => self.nodes[self.names[&x]].borrow().val << val,
            Gate::RShift(x, val) => self.nodes[self.names[&x]].borrow().val >> val
        };
        let neighbours = node.adj.to_owned();
        for n_name in neighbours.iter() {
            let mut neighbour = self.nodes[self.names[n_name]].borrow_mut();
            neighbour.ideg -= 1;
            if neighbour.ideg == 0 {
                queue.push(n_name.clone());
            }
        }
        node.val
    }

    fn toposort(&mut self) -> Vec<(String, u16)> {
        let mut ret   = Vec::new();
        let mut queue = Vec::new();

        self.recompute_indegrees();

        for (name, index) in self.names.iter() {
            if self.nodes[*index].borrow().ideg == 0 {
                queue.push(name.clone());
            }
        }

        while !queue.is_empty() {
            let name = queue[0].clone();
            let val = self.expand(name.clone(), &mut queue);
            ret.push((name, val));
            queue.swap_remove(0);
        }

        ret
    }
}

fn main() {
    let mut f = File::open(Path::new("/Users/PetarV/rust-proj/advent-of-rust/target/input.txt"))
    	.ok()
    	.expect("Failed to open the input file!");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.ok()
		.expect("Failed to read from the input file!");

    let mut graph = Graph::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        let gate: Gate;
        let output;

        if parts[0] == "NOT" {
            gate = Gate::Not(parts[1].to_string());
            output = parts[3];
        } else if parts[1] == "AND" {
            gate = Gate::And(parts[0].to_string(), parts[2].to_string());
            output = parts[4];
        } else if parts[1] == "OR" {
            gate = Gate::Or(parts[0].to_string(), parts[2].to_string());
            output = parts[4];
        } else if parts[1] == "LSHIFT" {
            gate = Gate::LShift(parts[0].to_string(), parts[2].parse().ok().expect("Could not parse into an integer!"));
            output = parts[4];
        } else if parts[1] == "RSHIFT" {
            gate = Gate::RShift(parts[0].to_string(), parts[2].parse().ok().expect("Could not parse into an integer!"));
            output = parts[4];
        } else { // const or assign
            gate = Gate::Const(parts[0].to_string());
            output = parts[2];
        }
        
        graph.add_connection(gate, output.to_string());
    }

    let mut sig_a = 0;

    for (id, val) in graph.toposort() {
        if id == "a" { 
            sig_a = val;
        }
    }

    println!("The signal on wire a is {}.", sig_a);

    graph.cut("b".to_string());
    graph.add_connection(Gate::ConstVal(sig_a), "b".to_string());

    for (id, val) in graph.toposort() {
        if id == "a" { 
            sig_a = val;
        }
    }

    println!("The new signal on wire a is {}.", sig_a);

}