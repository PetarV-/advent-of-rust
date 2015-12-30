use std::cmp::min;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

const INFTY: usize = 1 << 30;

fn main() {
    let mut f = File::open(Path::new("/Users/PetarV/rust-proj/advent-of-rust/target/input.txt"))
    	.ok()
    	.expect("Failed to open the input file!");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.ok()
		.expect("Failed to read from the input file!");

    let mut nums = Vec::new();

    let lim = 150;

    for line in input.lines() {
        let num: usize = line.parse().ok().expect("Could not parse into an integer!");
        nums.push(num);
    }

    let n = nums.len();

    let mut dp = vec![vec![0; lim + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..(n + 1) {
        for s in 0..(lim + 1) {
            if s < nums[i - 1] {
                dp[i][s] = dp[i - 1][s];
            } else {
                dp[i][s] = dp[i - 1][s] + dp[i - 1][s - nums[i - 1]];
            }
        }
    }

    println!("There are {} different combinations of containers.", dp[n][lim]);

    let mut dp = vec![INFTY; lim + 1];
    dp[0] = 0;

    for i in 0..n {
        for s in nums[i]..(lim + 1) {
            dp[s] = min(dp[s], dp[s - nums[i]] + 1);
        }
    }

    println!("The minimal amount of containers is {}.", dp[lim]);
}