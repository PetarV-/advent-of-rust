use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

#[derive(Clone)]
enum Instr { // been a while since enums were required
    Hlf(String),        // halve
    Tpl(String),        // triple
    Inc(String),        // increment
    Jmp(i32),           // jump
    Jie(String, i32),   // jump if even
    Jio(String, i32)    // jump if one
}

fn parse_offset(s: String) -> i32 {
    let mut rev: String = s.chars().rev().collect();
    let sign = rev.pop();
    rev = rev.chars().rev().collect();
    let offset: i32 = rev.parse().ok().expect("Could not parse into an integer!");
    if sign == Some('-') {
        -offset
    } else {
        offset
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

    let mut instrs = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts[0] == "hlf" {
            instrs.push(Instr::Hlf(parts[1].to_string()));
        } else if parts[0] == "tpl" {
            instrs.push(Instr::Tpl(parts[1].to_string()));
        } else if parts[0] == "inc" {
            instrs.push(Instr::Inc(parts[1].to_string()));
        } else if parts[0] == "jmp" {
            instrs.push(Instr::Jmp(parse_offset(parts[1].to_string())));
        } else if parts[0] == "jie" {
            let mut reg_str = parts[1].to_string(); reg_str.pop();
            instrs.push(Instr::Jie(reg_str, parse_offset(parts[2].to_string())));
        } else if parts[0] == "jio" {
            let mut reg_str = parts[1].to_string(); reg_str.pop();
            instrs.push(Instr::Jio(reg_str, parse_offset(parts[2].to_string())));
        }
    }

    let mut pc = 0 as i32;
    let mut state = HashMap::new();
    state.insert("a".to_string(), 0);
    state.insert("b".to_string(), 0);

    while 0 <= pc && pc < instrs.len() as i32 {
        match instrs[pc as usize].clone() {
            Instr::Hlf(x)      => {
                let curr = state[&x];
                state.insert(x, curr >> 1);
                pc += 1;
            },
            Instr::Tpl(x)      => {
                let curr = state[&x];
                state.insert(x, curr * 3);
                pc += 1;
            },
            Instr::Inc(x)      => {
                let curr = state[&x];
                state.insert(x, curr + 1);
                pc += 1;
            },
            Instr::Jmp(off)    => {
                pc += off;
            },
            Instr::Jie(x, off) => {
                if state[&x] & 1 == 0 {
                    pc += off;
                } else {
                    pc += 1;
                }
            },
            Instr::Jio(x, off) => {
                if state[&x] == 1 {
                    pc += off;
                } else {
                    pc += 1;
                }
            }
        }
    }

    println!("The final state of register b is {}.", state[&"b".to_string()]);

    state.insert("a".to_string(), 1);
    state.insert("b".to_string(), 0);
    pc = 0;

    while 0 <= pc && pc < instrs.len() as i32 {
        match instrs[pc as usize].clone() {
            Instr::Hlf(x)      => {
                let curr = state[&x];
                state.insert(x, curr >> 1);
                pc += 1;
            },
            Instr::Tpl(x)      => {
                let curr = state[&x];
                state.insert(x, curr * 3);
                pc += 1;
            },
            Instr::Inc(x)      => {
                let curr = state[&x];
                state.insert(x, curr + 1);
                pc += 1;
            },
            Instr::Jmp(off)    => {
                pc += off;
            },
            Instr::Jie(x, off) => {
                if state[&x] & 1 == 0 {
                    pc += off;
                } else {
                    pc += 1;
                }
            },
            Instr::Jio(x, off) => {
                if state[&x] == 1 {
                    pc += off;
                } else {
                    pc += 1;
                }
            }
        }
    }

    println!("Should a start at one, the final state of register b is {}.", state[&"b".to_string()]);
}