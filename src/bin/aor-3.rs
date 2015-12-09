use std::collections::HashSet;
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

    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut visited = HashSet::new();

    visited.insert((0, 0));

    for ch in input.chars() {
        match ch {
            'v' => pos_x = pos_x + 1,
            '>' => pos_y = pos_y + 1,
            '^' => pos_x = pos_x - 1,
            '<' => pos_y = pos_y - 1,
            _   => ()
        };
        visited.insert((pos_x, pos_y));
    }

    let ret = visited.len();

    println!("The Santa has visited {} houses.", ret);

    let mut pos_x = [0, 0];
    let mut pos_y = [0, 0];
    let mut visited = HashSet::new();

    visited.insert((0, 0));

    for (i, ch) in input.chars().enumerate() {
        let ind = i % 2;
        match ch {
            'v' => pos_x[ind] = pos_x[ind] + 1,
            '>' => pos_y[ind] = pos_y[ind] + 1,
            '^' => pos_x[ind] = pos_x[ind] - 1,
            '<' => pos_y[ind] = pos_y[ind] - 1,
            _   => ()
        };
        visited.insert((pos_x[ind], pos_y[ind]));
    }

    let ret = visited.len();

    println!("The Santa and Robo-Santa have visited {} houses.", ret);
}