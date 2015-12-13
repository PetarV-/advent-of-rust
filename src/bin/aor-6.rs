use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

// necessary in order to enforce copy rather than move semantics!
#[derive(Clone, Copy)]
enum Op {
    Nop,
    On,
    Off,
    Toggle(i32),
    MinusPlus(i32, i32)
}

struct TreeNode {
    val : i32,
    prop: Op
}

struct Quadtree {
    len_x: usize,
    len_y: usize,
    tree : Vec<TreeNode>
}

impl TreeNode {
    fn new() -> TreeNode {
        TreeNode {
            val : 0,
            prop: Op::Nop
        }
    }

    fn update(&mut self, op: Op) {
        // if let is pretty cool!
        if let Op::MinusPlus(x, y) = op {
            self.val -= x;
            if self.val < 0 {
                self.val = 0;
            }
            self.val += y;
            if let Op::MinusPlus(a, b) = self.prop {
                self.prop = if b >= x {
                    Op::MinusPlus(a, b - x + y)
                } else {
                    Op::MinusPlus(a + x - b, y)
                }
            } else {
                self.prop = op;
            }
        } else {
            match op {
                Op::On        => { 
                    self.val  = 1; 
                    self.prop = Op::On;  
                },
                Op::Off       => { 
                    self.val  = 0; 
                    self.prop = Op::Off; 
                },
                Op::Toggle(x) => { 
                    self.val = (self.val + x) % 2;
                    match self.prop {
                        Op::On        => { 
                            if x % 2 == 1 {
                                self.prop = Op::Off;
                            } else {
                                self.prop = Op::On;
                            }
                        },
                        Op::Off       => { 
                            if x % 2 == 1 {
                                self.prop = Op::On;
                            } else {
                                self.prop = Op::Off;
                            }
                        },
                        Op::Toggle(y) => {
                            self.prop = Op::Toggle(x + y);
                        },
                        _             => { 
                            self.prop = op; 
                        }
                    }
                },
                _             => { }
            };
        } 
    }
}

impl Quadtree {
    fn new(len_x: usize, len_y: usize) -> Quadtree {
        let size = (len_x * len_y) << 2; // reserve at least 4*N*M nodes
        Quadtree {
            len_x: len_x,
            len_y: len_y,
            tree : (0..size).map(|_i| { 
                TreeNode::new()
            }).collect()
        }
    }

    fn propagate(&mut self, idx: usize) {
        let op = self.tree[idx].prop;
        for i in 1..5 {
            let chd = (idx << 2) + i;
            if chd < self.tree.len() {
                self.tree[chd].update(op);
            }
        }
        self.tree[idx].prop = Op::Nop;
    }

    fn core_update(&mut self, idx: usize, l: usize, r: usize, u: usize, d: usize, op: Op, left: usize, right: usize, up: usize, down: usize) {
        self.propagate(idx);
        if l <= left && right <= r && u <= up && down <= d {
            self.tree[idx].update(op);
        } else {
            let mid_x = (left + right) >> 1;
            let mid_y = (up + down) >> 1;
            if l <= mid_x && u <= mid_y {
                self.core_update((idx << 2) + 1, l, r, u, d, op, left, mid_x, up, mid_y);
            }
            if r > mid_x && u <= mid_y {
                self.core_update((idx << 2) + 2, l, r, u, d, op, mid_x + 1, right, up, mid_y);
            }
            if l <= mid_x && d > mid_y {
                self.core_update((idx << 2) + 3, l, r, u, d, op, left, mid_x, mid_y + 1, down);
            }
            if r > mid_x && d > mid_y {
                self.core_update((idx << 2) + 4, l, r, u, d, op, mid_x + 1, right, mid_y + 1, down);
            }
        }
    }

    fn core_query(&mut self, idx: usize, left: usize, right: usize, up: usize, down: usize) -> i32 {
        self.propagate(idx);
        if left == right && up == down {
            self.tree[idx].val
        } else {
            let mid_x = (left + right) >> 1;
            let mid_y = (up + down) >> 1;
            let top_left = self.core_query((idx << 2) + 1, left, mid_x, up, mid_y);
            let top_right = if mid_x < right {
                self.core_query((idx << 2) + 2, mid_x + 1, right, up, mid_y)
            } else {
                0
            };
            let bottom_left = if mid_y < down {
                self.core_query((idx << 2) + 3, left, mid_x, mid_y + 1, down)
            } else {
                0
            };
            let bottom_right = if mid_x < right && mid_y < down {
                self.core_query((idx << 2) + 4, mid_x + 1, right, mid_y + 1, down)
            } else {
                0
            };
            top_left + top_right + bottom_left + bottom_right
        }
    }

    fn update(&mut self, l: usize, r: usize, u:usize, d:usize, op: Op) {
        /* 
         must compute separately, as the borrows checker
         concludes that self.len could get invalidated
        */
        let max_x = self.len_x - 1;
        let max_y = self.len_y - 1;
        self.core_update(0, l, r, u, d, op, 0, max_x, 0, max_y);
    }

    fn query(&mut self) -> i32 {
        let max_x = self.len_x - 1;
        let max_y = self.len_y - 1;
        self.core_query(0, 0, max_x, 0, max_y)
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

    let mut qtree = Quadtree::new(1000, 1000);

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let lu: Vec<_>;
        let rd: Vec<_>;
        let op;

        if parts[0] == "toggle" {
            lu = parts[1].split(',').collect();
            rd = parts[3].split(',').collect();
            op = Op::Toggle(1);
        } else {
            lu = parts[2].split(',').collect();
            rd = parts[4].split(',').collect();
            if parts[1] == "on" {
                op = Op::On;
            } else {
                op = Op::Off;
            }
        }
        let l = lu[0].parse().ok().expect("Could not parse into an integer!");
        let u = lu[1].parse().ok().expect("Could not parse into an integer!");
        let r = rd[0].parse().ok().expect("Could not parse into an integer!");
        let d = rd[1].parse().ok().expect("Could not parse into an integer!");

        qtree.update(l, r, u, d, op);
    }

    let ret = qtree.query();

    println!("{} lights are turned on at the end.", ret);

    let mut qtree = Quadtree::new(1000, 1000);

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        let lu: Vec<_>;
        let rd: Vec<_>;
        let op;

        if parts[0] == "toggle" {
            lu = parts[1].split(',').collect();
            rd = parts[3].split(',').collect();
            op = Op::MinusPlus(0, 2);
        } else {
            lu = parts[2].split(',').collect();
            rd = parts[4].split(',').collect();
            if parts[1] == "on" {
                op = Op::MinusPlus(0, 1);
            } else {
                op = Op::MinusPlus(1, 0);
            }
        }
        let l = lu[0].parse().ok().expect("Could not parse into an integer!");
        let u = lu[1].parse().ok().expect("Could not parse into an integer!");
        let r = rd[0].parse().ok().expect("Could not parse into an integer!");
        let d = rd[1].parse().ok().expect("Could not parse into an integer!");

        qtree.update(l, r, u, d, op);
    }

    let ret = qtree.query();

    println!("The total brightness of the lights is {}.", ret);
}