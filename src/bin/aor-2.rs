use std::cmp;
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

    // We're reading the input line by line now, so create a buffered reader
    let ret = input.lines().fold(0, |acc, line| {
    	let sides: Vec<u32> = line.split('x').map(|len| { 
    			len.parse().ok().expect("Could not parse into an integer!")
    		}).collect();

    	let mut full = 0;
    	let mut slack = sides[0] * sides[1];

    	for (i, s1) in sides.iter().enumerate() {
    		for s2 in sides[i+1..].iter() {
    			let curr = s1 * s2;
    			full = full + (curr << 1);
    			slack = cmp::min(slack, curr);
    		}
    	}

    	acc + full + slack
    });

    println!("The total surface of the wrapping paper is {} square feet.", ret);

    let ret = input.lines().fold(0, |acc, line| {
    	let mut sides: Vec<u32> = line.split('x').map(|len| { 
    			len.parse().ok().expect("Could not parse into an integer!")
    		}).collect();

    	sides.sort();

    	acc + 2 * sides[0] + 2 * sides[1] + sides[0] * sides[1] * sides[2]
    });

    println!("The total length of the ribbon is {} feet.", ret);
}