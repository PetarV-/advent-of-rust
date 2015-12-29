use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::cell::RefCell;

enum State {
    Normal,
    Minus,
    Number(i32),
    NegNum(i32)
}

enum SuperState { // now we specify also the object depth we are in
    // Depth zero - we don't care about "red"
    Normal,
    // in obj, amassed sum x, stack of (isObj/sums)
    Obj(i32, RefCell<Vec<(bool, i32)>>),
    // Within an array that is within an obj - don't care about red for now
    ObjArr(i32, RefCell<Vec<(bool, i32)>>),
    // in obj, reading value for potential red, x: scanning?, y: chars into red
    ObjVal(i32, usize, RefCell<Vec<(bool, i32)>>),
    // in red obj caught at depth x from now, stored stack of (isObj/sums)
    RedObj(usize, RefCell<Vec<(bool, i32)>>)
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

    for line in input.lines() {
        let (sum, st) = line.chars()
            .fold((0, State::Normal), |(acc, st), ch| {
                match st {
                    State::Normal    => if ch == '-' {
                        (acc, State::Minus)
                    } else if ch.is_digit(10) {
                        let dig = ch.to_digit(10).expect("Failed converting to digit!") as i32;
                        (acc, State::Number(dig))
                    } else {
                        (acc, State::Normal)
                    },
                    State::Minus     => if ch == '-' { // it is not really clear what to do here
                        (acc, State::Normal)
                    } else if ch.is_digit(10) {
                        let dig = ch.to_digit(10).expect("Failed converting to digit!") as i32;
                        (acc, State::NegNum(dig))
                    } else {
                        (acc, State::Normal)
                    },
                    State::Number(n) => if ch == '-' { // here either
                        (acc + n, State::Normal)
                    } else if ch.is_digit(10) {
                        let dig = ch.to_digit(10).expect("Failed converting to digit!") as i32;
                        (acc, State::Number(n * 10 + dig))
                    } else {
                        (acc + n, State::Normal)
                    },
                    State::NegNum(n) => if ch == '-' { // here either
                        (acc - n, State::Normal)
                    } else if ch.is_digit(10) {
                        let dig = ch.to_digit(10).expect("Failed converting to digit!") as i32;
                        (acc, State::NegNum(n * 10 + dig))
                    } else {
                        (acc - n, State::Normal)
                    }
                }
            });
        ret += sum + match st {
            State::Number(n) => n,
            State::NegNum(n) => -n,
            _                => 0
        };
    }

    println!("The sum of all numbers within the JSON is {}.", ret);

    let key = "\"red\"";

    ret = 0;

    for line in input.lines() {
        let (_st, _sup) = line.chars()
            .fold((State::Normal, SuperState::Normal), |(st, sup), ch| {
                let (to_add, next_st) = match st {
                    State::Normal    => if ch == '-' {
                        (0, State::Minus)
                    } else if ch.is_digit(10) {
                        let dig = ch.to_digit(10).expect("Failed converting to digit!") as i32;
                        (0, State::Number(dig))
                    } else {
                        (0, State::Normal)
                    },
                    State::Minus     => if ch == '-' { // it is not really clear what to do here
                        (0, State::Normal)
                    } else if ch.is_digit(10) {
                        let dig = ch.to_digit(10).expect("Failed converting to digit!") as i32;
                        (0, State::NegNum(dig))
                    } else {
                        (0, State::Normal)
                    },
                    State::Number(n) => if ch == '-' { // here either
                        (n, State::Normal)
                    } else if ch.is_digit(10) {
                        let dig = ch.to_digit(10).expect("Failed converting to digit!") as i32;
                        (0, State::Number(n * 10 + dig))
                    } else {
                        (n, State::Normal)
                    },
                    State::NegNum(n) => if ch == '-' { // here either
                        (-n, State::Normal)
                    } else if ch.is_digit(10) {
                        let dig = ch.to_digit(10).expect("Failed converting to digit!") as i32;
                        (0, State::NegNum(n * 10 + dig))
                    } else {
                        (-n, State::Normal)
                    }
                };

                let next_sup = match sup {
                    SuperState::Normal => match ch {
                        '{' => SuperState::Obj(0, RefCell::new(Vec::new())),
                        '[' => SuperState::ObjArr(0, RefCell::new(Vec::new())),
                        _   => {
                            ret += to_add; 
                            SuperState::Normal
                        }
                    },
                    SuperState::Obj(sum, ref_stack) => match ch {
                        '}' => {
                            let top;
                            {
                                top = ref_stack.borrow_mut().pop();
                            }
                            match top {
                                Some((is_obj, top_sum)) => if is_obj {
                                    SuperState::Obj(top_sum + sum + to_add, ref_stack)
                                } else {
                                    SuperState::ObjArr(top_sum + sum + to_add, ref_stack)
                                },
                                None => {
                                    ret += sum + to_add;
                                    SuperState::Normal
                                }
                            }
                        },
                        ':' => SuperState::ObjVal(sum, 0, ref_stack),
                        _   => SuperState::Obj(sum + to_add, ref_stack)
                    },
                    SuperState::ObjArr(sum, ref_stack) => match ch {
                        '{' => { 
                            ref_stack.borrow_mut().push((false, sum));
                            SuperState::Obj(0, ref_stack)
                        },
                        '[' => {
                            ref_stack.borrow_mut().push((false, sum));
                            SuperState::ObjArr(0, ref_stack)
                        },
                        ']' => {
                            let top;
                                {
                                    top = ref_stack.borrow_mut().pop();
                                }
                                match top {
                                Some((is_obj, top_sum)) => if is_obj {
                                    SuperState::Obj(top_sum + sum + to_add, ref_stack)
                                } else {
                                    SuperState::ObjArr(top_sum + sum + to_add, ref_stack)
                                },
                                None => {
                                    ret += sum + to_add;
                                    SuperState::Normal
                                }
                            }
                        },
                        _   => SuperState::ObjArr(sum + to_add, ref_stack)
                    },
                    SuperState::ObjVal(sum, index, ref_stack) => if ch as u8 == key.as_bytes()[index] {
                        if index == key.len() - 1 {
                            SuperState::RedObj(0, ref_stack)
                        } else {
                            SuperState::ObjVal(sum, index + 1, ref_stack)
                        }
                    } else {
                        match ch {
                            '{' => { 
                                ref_stack.borrow_mut().push((true, sum));
                                SuperState::Obj(0, ref_stack)
                            },
                            '[' => {
                                ref_stack.borrow_mut().push((true, sum));
                                SuperState::ObjArr(0, ref_stack)
                            },
                            _   => SuperState::Obj(sum + to_add, ref_stack)
                        }
                    },
                    SuperState::RedObj(depth, ref_stack) => match ch {
                        '{' => SuperState::RedObj(depth + 1, ref_stack),
                        '}' => if depth == 0 {
                            let top;
                            {
                                top = ref_stack.borrow_mut().pop();
                            }
                            match top {
                                Some((is_obj, top_sum)) => if is_obj {
                                    SuperState::Obj(top_sum, ref_stack)
                                } else {
                                    SuperState::ObjArr(top_sum, ref_stack)
                                },
                                None => {
                                    SuperState::Normal
                                }
                            }
                        } else {
                            SuperState::RedObj(depth - 1, ref_stack)
                        },
                        _  => SuperState::RedObj(depth, ref_stack)
                    }
                };
                (next_st, next_sup)
        });
    }

    println!("The sum of all valid numbers within the JSON is {}.", ret);
}