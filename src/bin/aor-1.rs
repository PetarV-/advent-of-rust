use std::io::prelude::*;
use std::fs::File; // we will be reading the input from a file
use std::path::Path;

fn main() {
	//let mut f = try!(File::open("input.txt")); // try! macro wraps error handling

	// Open the path in read-only mode, returns `io::Result<File>`
    let mut f = File::open(Path::new("/Users/PetarV/rust-proj/advent-of-rust/target/input.txt"))
    	.ok()
    	.expect("Failed to open the input file!");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.ok()
		.expect("Failed to read from the input file!");

	let ret = input.chars().fold(0, |acc, ch| {
		match ch {
			'(' => acc + 1,
			')' => acc - 1,
			_   => acc
		}
	});

	println!("The Santa will end up at floor {}", ret);

	let mut floor = 0;

	let ret = input.chars().position(|ch| {
		floor = match ch {
			'(' => floor + 1,
			')' => floor - 1,
			_   => floor
		};
		floor == -1
	}).expect("Failed to find the required position!"); 

	println!("The Santa will first enter the basement at instruction {}", ret + 1);
}