use std::cmp::min;
use std::cmp::max;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::cell::RefCell;

const INFTY: i32 = 1 << 30;

struct Node {
    adj: Vec<(String, i32)>
}

struct Graph {
    names: HashMap<String, usize>,
    nodes: Vec<RefCell<Node>>
}

impl Node {
    fn new() -> Node {
        Node {
            adj: Vec::new()
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

    fn add_edge(&mut self, from: String, to: String, dist: i32) {
        self.spawn_if_missing(from.clone());
        self.spawn_if_missing(to.clone());
        let mut node_a = self.nodes[self.names[&from]].borrow_mut();
        node_a.adj.push((to.clone(), dist));
    }

    fn get_tsp_length(&self) -> i32 {
        let n = self.nodes.len();
        let lim = 1 << n;
        let mut dp = vec![vec![-INFTY; n]; lim];
        dp[1][0] = 0;
        for mask in 2..lim {
            if mask % 2 == 1 {
                for i in 1..n {
                    if (mask >> i) & 1 == 1 {
                        let neighbours = self.nodes[i].borrow().adj.clone();
                        for neighbour in neighbours.iter() {
                            let j = self.names[&neighbour.0];
                            let weight = neighbour.1;
                            if (mask >> j) & 1 == 1 {
                                dp[mask][i] = max(dp[mask][i], dp[mask ^ (1 << i)][j] + weight);
                            }
                        }
                    }
                }
            }
        }
        let mut ret = -INFTY;
        let neighbours = self.nodes[0].borrow().adj.clone();
        for neighbour in neighbours.iter() {
            let i = self.names[&neighbour.0];
            let weight = neighbour.1;
            ret = max(ret, dp[lim - 1][i] + weight);
        }
        ret
    }

    fn get_longest_ham_length(&self) -> i32 {
        let n = self.nodes.len();
        let lim = 1 << n;
        let mut dp = vec![vec![-INFTY; n]; lim];
        for i in 0..n {
            dp[1 << i][i] = 0;
        }
        for mask in 0..lim {
            for i in 0..n {
                if (mask >> i) & 1 == 1 {
                    let neighbours = self.nodes[i].borrow().adj.clone();
                    for neighbour in neighbours.iter() {
                        let j = self.names[&neighbour.0];
                        let weight = neighbour.1;
                        if (mask >> j) & 1 == 1 {
                            dp[mask][i] = max(dp[mask][i], dp[mask ^ (1 << i)][j] + weight);
                        }
                    }
                }
            }
        }
        let mut ret = -INFTY;
        for i in 0..n {
            ret = max(ret, dp[lim - 1][i]);
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
    let mut edges: HashMap<(String, String), i32> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        let a = parts[0].to_string();
        let g = parts[2].to_string();
        let d: i32 = parts[3].to_string().parse().ok().expect("Could not parse into an integer!");
        let mut b = parts[10].to_string();
        b.pop();

        let w = if g == "gain" {
            d
        } else if g == "lose" {
            -d
        } else {
            panic!("Something is wrong, no gain/lose in line!");
        };

        let key = (min(a.clone(), b.clone()), max(a.clone(), b.clone()));
        if edges.contains_key(&key) {
            let val = edges[&key];
            edges.insert(key, val + w);
        } else {
            edges.insert(key, w);
        }
    }

    for (key, w) in &edges {
        let (a, b) = key.clone();
        graph.add_edge(a.clone(), b.clone(), *w);
        graph.add_edge(b.clone(), a.clone(), *w);
    }

    let ret = graph.get_tsp_length();

    println!("The longest Hamiltonian cycle length is {}.", ret);

    let ret = graph.get_longest_ham_length();

    println!("The longest Hamiltonian path length is {}.", ret);
}