use std::cmp::min;
use std::cmp::max;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Clone)]
struct OhDeer {
    speed: u32,
    t_fly: u32,
    t_rst: u32,
    pts  : u32
}

impl OhDeer {
    fn new(speed: u32, t_fly: u32, t_rst: u32, pts: u32) -> OhDeer {
        OhDeer {
            speed: speed,
            t_fly: t_fly,
            t_rst: t_rst, 
            pts  : pts
        }
    }

    fn travelled(&self, t: u32) -> u32 {
        let t_cyc = self.t_fly + self.t_rst;
        let cycles = t / t_cyc;
        let rem = min(self.t_fly, t % t_cyc);
        self.speed * (cycles * self.t_fly + rem)  
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

    let mut ret = 0;

    let mut deers = Vec::new();

    let t_lim = 2503;

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();

        let speed = parts[3].to_string().parse().ok().expect("Could not parse into an integer!");
        let t_fly = parts[6].to_string().parse().ok().expect("Could not parse into an integer!");
        let t_rst = parts[13].to_string().parse().ok().expect("Could not parse into an integer!"); 

        let deer = OhDeer::new(speed, t_fly, t_rst, 0);

        deers.push(deer.clone());
        ret = max(ret, deer.travelled(t_lim));
    }

    println!("The winning reindeer has travelled {} kilometers.", ret);

    for t in 0..t_lim {
        let mut max_dist = 0;
        for deer in deers.iter() {
            let curr = deer.travelled(t + 1);
            max_dist = max(max_dist, curr);
        }
        for deer in deers.iter_mut() {
            if deer.travelled(t + 1) == max_dist {
                deer.pts += 1;
            }
        }
    }

    let ret = deers.iter().fold(0, |curr, deer| {
        max(curr, deer.pts)
    });

    println!("The winning reindeer has won {} points.", ret);
}