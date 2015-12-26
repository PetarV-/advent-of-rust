use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut f = File::open(Path::new("/Users/PetarV/rust-proj/advent-of-rust/target/input.txt"))
    	.ok()
    	.expect("Failed to open the input file!");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.ok()
		.expect("Failed to read from the input file!");

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let illegal = ["ab", "cd", "pq", "xy"];

    let ret = input.lines().fold(0, |acc, line| {
        let vowel_cnt = line.chars().fold(0, |acc, ch| {
            if vowels.contains(&ch) {
                acc + 1
            } else {
                acc
            }
        });

        let has_twice = line.chars().collect::<Vec<_>>()
            .windows(2)
            .fold(false, |acc, win| {
                acc || win[0] == win[1]
            });

        let has_illegal = illegal.iter().fold(false, |acc, ill| {
            acc || line.contains(ill)
        });

        if vowel_cnt >= 3 && has_twice && !has_illegal {
            acc + 1
        } else {
            acc
        }
    });

    println!("There are {} nice strings.", ret);

    let ret = input.lines().fold(0, |acc, line| {
        let mut hmap = HashMap::new();

        let cond1 = line.chars().collect::<Vec<_>>()
            .windows(2).enumerate()
            .fold(false, |acc, (i, win)| {
                acc || if hmap.contains_key(win) {
                    hmap[win] != i - 1
                } else {
                    hmap.insert(win.to_vec(), i); 
                    false
                }
            });

        let cond2 = line.chars().collect::<Vec<_>>().windows(3)
            .fold(false, |acc, win| {
                acc || win[0] == win[2]
            });

        if cond1 && cond2 {
            acc + 1
        } else {
            acc
        }
    });

    println!("There are {} nice strings under the new rules.", ret);
}