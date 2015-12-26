use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

enum State {
    Normal,
    Backslash,
    ParseHex(bool)
}

fn main() {
    let mut f = File::open(Path::new("/Users/PetarV/rust-proj/advent-of-rust/target/input.txt"))
    	.ok()
    	.expect("Failed to open the input file!");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.ok()
		.expect("Failed to read from the input file!");

    let mut chars_in_code = 0;
    let mut chars_in_memo = 0;

    for line in input.lines() {
        chars_in_code += line.len();
        chars_in_memo += line.chars()
            .fold((0, State::Normal), |(acc, st), ch| {
                match st {
                    State::Normal      => match ch {
                        '"'  => (acc, State::Normal),
                        '\\' => (acc + 1, State::Backslash),
                        _    => (acc + 1, State::Normal)
                    },
                    State::Backslash   => match ch {
                        'x'  => (acc, State::ParseHex(true)),
                        _    => (acc, State::Normal)
                    },
                    State::ParseHex(b) => if b {
                        (acc, State::ParseHex(false))
                    } else {
                        (acc, State::Normal)
                    }
                }
            }).0;
    }

    let ret = chars_in_code - chars_in_memo;

    println!("The difference in characters in code and memory is {}.", ret);

    let mut chars_encoded = 0;

    for line in input.lines() {
        chars_encoded += line.chars().fold(2, |acc, ch| { // start at 2 to entail endquotes
            match ch {
                '"'  => acc + 2,
                '\\' => acc + 2,
                _    => acc + 1
            }
        });
    }

    let ret = chars_encoded - chars_in_code;

    println!("The difference in code characters in the encoded and original version is {}.", ret);
}