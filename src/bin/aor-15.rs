use std::cmp::max;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

struct Ingr {
    cap: i32,
    dur: i32,
    flv: i32,
    txt: i32,
    cal: i32
}

impl Ingr {
    fn new(cap: i32, dur: i32, flv: i32, txt: i32, cal: i32) -> Ingr {
        Ingr {
            cap: cap,
            dur: dur,
            flv: flv,
            txt: txt,
            cal: cal
        }
    }

    fn score(&self) -> i32 {
        max(0, self.cap) * max(0, self.dur) * max(0, self.flv) * max(0, self.txt)
    }
}

fn partition(ingrs: &Vec<Ingr>, tot: usize, cal_lim: i32) -> i32 {
    fn partition_internal(ingrs: &Vec<Ingr>, index: usize, rem: usize, cal_lim: i32, stored: &mut Vec<usize>) -> i32 {
        if index == ingrs.len() - 1 {
            stored.push(rem);
            let super_ingr = ingrs.iter().enumerate().fold(Ingr::new(0, 0, 0, 0, 0), |acc, (i, ingr)| {
                Ingr::new(acc.cap + stored[i] as i32 * ingr.cap,
                          acc.dur + stored[i] as i32 * ingr.dur,
                          acc.flv + stored[i] as i32 * ingr.flv,
                          acc.txt + stored[i] as i32 * ingr.txt,
                          acc.cal + stored[i] as i32 * ingr.cal)
            });
            stored.pop();
            if cal_lim == -1 || super_ingr.cal == cal_lim {
                super_ingr.score()
            } else {
                0
            }
        } else {
            let mut ret = 0;
            for amt in 0..(rem + 1) {
                stored.push(amt);
                ret = max(ret, partition_internal(&ingrs, index + 1, rem - amt, cal_lim, stored));
                stored.pop();
            }
            ret
        }
    };
    partition_internal(ingrs, 0, tot, cal_lim, &mut Vec::new())
}

fn main() {
    let mut f = File::open(Path::new("/Users/PetarV/rust-proj/advent-of-rust/target/input.txt"))
    	.ok()
    	.expect("Failed to open the input file!");

	let mut input = String::new();
	f.read_to_string(&mut input)
		.ok()
		.expect("Failed to read from the input file!");

    let tot = 100;
    let mut ingrs = Vec::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        let mut cap_str = parts[2].to_string().clone(); cap_str.pop();
        let mut dur_str = parts[4].to_string().clone(); dur_str.pop();
        let mut flv_str = parts[6].to_string().clone(); flv_str.pop();
        let mut txt_str = parts[8].to_string().clone(); txt_str.pop();
        let cal_str = parts[10];

        let cap = cap_str.parse().ok().expect("Could not parse into an integer!");
        let dur = dur_str.parse().ok().expect("Could not parse into an integer!");
        let flv = flv_str.parse().ok().expect("Could not parse into an integer!");
        let txt = txt_str.parse().ok().expect("Could not parse into an integer!");
        let cal = cal_str.parse().ok().expect("Could not parse into an integer!");

        ingrs.push(Ingr::new(cap, dur, flv, txt, cal));
    }   

    let ret = partition(&ingrs, tot, -1);

    println!("The optimal total score is {}.", ret);

    let cal_lim = 500;

    let ret = partition(&ingrs, tot, cal_lim);

    println!("The optimal total score with the calory constraint is {}.", ret);
}