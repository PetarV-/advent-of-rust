use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

struct Nonterminals {
    map : HashMap<String, u32>,
    symb: Vec<String>
}

#[derive(Clone)]
struct Rule { // Chomsky Normal Form: all rules expressed as A -> BC
    lhs : u32,
    rhs : (u32, u32),
    cost: u32
}

impl Nonterminals {
    fn new() -> Nonterminals {
        Nonterminals {
            map : HashMap::new(),
            symb: Vec::new()
        }
    }

    fn get_id(&mut self, s: String) -> u32 { // Precondition: s is a single symbol
        if self.map.contains_key(&s) {
            self.map[&s]
        } else {
            let id = self.symb.len();
            self.map.insert(s.clone(), id as u32);
            self.symb.push(s);
            id as u32
        }
    }

    fn get_ids(&mut self, s: String) -> Vec<u32> {
        let mut ret = Vec::new();
        let mut curr = "".to_string();
        for ch in s.chars() {
            if ch.is_uppercase() {
                if !curr.is_empty() {
                    ret.push(self.get_id(curr));
                }
                curr = ch.to_string();
            } else {
                curr.push(ch);
            }
        }
        if !curr.is_empty() {
            ret.push(self.get_id(curr));
        }
        ret
    }
}

impl Rule {
    fn new(lhs: u32, rhs: (u32, u32), cost: u32) -> Rule { // Basic constructor
        Rule {
            lhs : lhs,
            rhs : rhs,
            cost: cost
        }
    }

    fn to_cnf(lhs: String, rhs: String, nts: &mut Nonterminals) -> Vec<Rule> {
        let mut ret = Vec::new();
        let lhs_id = nts.get_id(lhs.clone());
        let rhs_parts = nts.get_ids(rhs.clone());
        if rhs_parts.len() == 2 {
            ret.push(Rule::new(lhs_id, (rhs_parts[0], rhs_parts[1]), 1));
        } else {
            let temp_rule = lhs + &rhs;
            let mut curr_lhs = lhs_id;
            let mut cost = 1;
            for (i, sym) in rhs_parts.iter().enumerate() {
                if i < rhs_parts.len() - 2 {
                    let curr_rule = nts.get_id(temp_rule.clone() + &i.to_string());
                    ret.push(Rule::new(curr_lhs, (*sym, curr_rule), cost));
                    curr_lhs = curr_rule;
                    cost = 0;
                } else {
                    break;
                }
            }
            ret.push(Rule::new(curr_lhs, (rhs_parts[rhs_parts.len() - 2], rhs_parts[rhs_parts.len() - 1]), cost));
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

    let mut rules = Vec::new();
    let mut orig = Vec::new();
    let mut nts = Nonterminals::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 3 { // Rule
            rules.append(&mut Rule::to_cnf(parts[0].to_string(), parts[2].to_string(), &mut nts));
        } else if !parts.is_empty() { // Base string
            orig = nts.get_ids(parts[0].to_string());
        }
    }

    // Need the inserted vectors to live longer than the hashset, so generating them first
    let extensions: Vec<_> = rules.iter().map(|rule| vec![rule.rhs.0, rule.rhs.1]).collect();

    let mut poss = HashSet::new();

    /*
     Decided to iterate over all rules rather than precompute which rules belong to which LHS.
     It makes the code above look much prettier, does not modify the worst-case running time,
     and is way less cumbersome.
    */
    for (i, sym) in orig.iter().enumerate() {
        for (j, rule) in rules.iter().enumerate() {
            if rule.lhs == *sym {
                let new_str: Vec<_> = orig.iter().take(i)
                    .chain(extensions[j].iter())
                    .chain(orig.iter().skip(i + 1))
                    .collect();
                poss.insert(new_str);
            }
        }
    }

    println!("There are {} different ways to transform the molecule with one rule.", poss.len());

    let n = orig.len();
    let r = nts.symb.len();
    let mut p = vec![vec![vec![-1; r]; n]; n + 1];

    for (i, sym) in orig.iter().enumerate() {
        p[1][i as usize][*sym as usize] = 0;
    }
    for i in 2..n + 1 {
        for j in 0..n - i + 1 {
            for k in 1..i {
                for rule in rules.iter() {
                    if p[k][j][rule.rhs.0 as usize] > -1 && p[i-k][j+k][rule.rhs.1 as usize] > -1 {
                        let new_val = p[k][j][rule.rhs.0 as usize] + p[i-k][j+k][rule.rhs.1 as usize] + rule.cost as i32;
                        if p[i][j][rule.lhs as usize] == -1 || p[i][j][rule.lhs as usize] > new_val {
                            p[i][j][rule.lhs as usize] = new_val;
                        }
                    }
                }
            }
        }
    }

    println!("The optimal derivation has length {}.", p[n][0][nts.get_id("e".to_string()) as usize]);
}