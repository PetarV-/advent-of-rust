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
    let mut sum = 0;

    for line in input.lines() {
        let num: usize = line.parse().ok().expect("Could not parse into an integer!");
        nums.push(num);
        sum += num;
    }

    nums.sort();
    nums.reverse();
    let n = nums.len();
    let lim = sum / 3;

    let mut dp = vec![vec![INFTY; lim + 1]; n + 1];
    let mut used = vec![vec![false; lim + 1]; n + 1];

    dp[0][0] = 0;
    for i in 1..(n + 1) {
        for s in 0..(lim + 1) {
            if s < nums[i - 1] {
                dp[i][s] = dp[i - 1][s];
            } else {
                dp[i][s] = if dp[i - 1][s] < dp[i - 1][s - nums[i - 1]] + 1 {
                    dp[i - 1][s]
                } else {
                    used[i][s] = true;
                    dp[i - 1][s - nums[i - 1]] + 1
                }
            }
        }
    }

    let mut ret = 1;
    let mut i = n;
    let mut s = lim;

    while s > 0 {
        if used[i][s] {
            ret *= nums[i - 1];
            s -= nums[i - 1];
        }
        i -= 1;
    }

    println!("The optimal quantum entanglement is {}.", ret);

    let lim = sum / 4;

    let mut dp = vec![vec![INFTY; lim + 1]; n + 1];
    let mut used = vec![vec![false; lim + 1]; n + 1];

    dp[0][0] = 0;
    for i in 1..(n + 1) {
        for s in 0..(lim + 1) {
            if s < nums[i - 1] {
                dp[i][s] = dp[i - 1][s];
            } else {
                dp[i][s] = if dp[i - 1][s] < dp[i - 1][s - nums[i - 1]] + 1 {
                    dp[i - 1][s]
                } else {
                    used[i][s] = true;
                    dp[i - 1][s - nums[i - 1]] + 1
                }
            }
        }
    }

    let mut ret = 1;
    let mut i = n;
    let mut s = lim;

    while s > 0 {
        if used[i][s] {
            ret *= nums[i - 1];
            s -= nums[i - 1];
        }
        i -= 1;
    }

    println!("The optimal quantum entanglement is now {}.", ret);
}