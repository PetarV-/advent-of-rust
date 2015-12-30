use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn mod_pow(num: u64, pow: u64, modulo: u64) -> u64 {
    let mut num = num;
    let mut pow = pow;
    let mut ret = 1;
    while pow > 0 {
        if pow & 1 == 1 {
            ret = (ret * num) % modulo;
        }
        pow >>= 1;
        num = (num * num) % modulo;
    }
    return ret;
}

fn main() {
    let mut f = File::open(Path::new("/Users/PetarV/rust-proj/advent-of-rust/target/input.txt"))
    	.ok()
    	.expect("Failed to open the input file!");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.ok()
		.expect("Failed to read from the input file!");

    let code = 20151125;
    let mult = 252533;
    let modu = 33554393;

    let mut row: u64 = 0;
    let mut col: u64 = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 18 {
            let mut row_str = parts[15].to_string(); row_str.pop();
            let mut col_str = parts[17].to_string(); col_str.pop();
            row = row_str.parse().ok().expect("Could not parse into an integer!");
            col = col_str.parse().ok().expect("Could not parse into an integer!");
        }
    }

    let diag = row + col - 1;
    let exp = ((diag * (diag - 1)) >> 1) + col - 1;

    let ret = code * mod_pow(mult, exp, modu) % modu;

    println!("The element in the required position is {}.", ret);

}