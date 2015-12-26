use std::collections::HashMap;

fn increment(vec: &mut Vec<u8>) {
    let mut stop = false;
    vec.reverse();
    for x in vec.iter_mut() {
        if *x == 122 { // 'z'
            *x = 97; // 'a'
        } else {
            *x += 1;
            stop = true;
            break;
        }
    }
    if !stop {
        vec.push(97); // append 'a' to front
    }
    vec.reverse();
}

fn main() {
    let mut password = "cqjxjnds".to_string();

    let mut vec = password.clone().into_bytes();

    loop {
        increment(&mut vec);

        password = String::from_utf8(vec.clone()).unwrap();

        let mut hmap = HashMap::new();

        let cond1 = vec.windows(3)
            .fold(false, |acc, win| {
                acc || win[0] + 1 == win[1] && win[1] + 1 == win[2]
            });

        let cond2 = password.chars()
            .fold(true, |acc, ch| {
                acc && ch != 'i' && ch != 'o' && ch != 'l'
            });

        let cond3 = vec.windows(2).enumerate()
            .fold(0, |acc, (i, win)| {
                acc + if win[0] == win[1] {
                    if hmap.contains_key(win) {
                        if hmap[win] != i - 1 {
                            1
                        } else {
                            0
                        }
                    } else {
                        hmap.insert(win.to_vec(), i);
                        1
                    }
                } else {
                    0
                }
            }) >= 2;

        if cond1 && cond2 && cond3 {
            break;
        }
    }

    println!("The next password is {}.", password);

    loop {
        increment(&mut vec);

        password = String::from_utf8(vec.clone()).unwrap();

        let mut hmap = HashMap::new();

        let cond1 = vec.windows(3)
            .fold(false, |acc, win| {
                acc || win[0] + 1 == win[1] && win[1] + 1 == win[2]
            });

        let cond2 = password.chars()
            .fold(true, |acc, ch| {
                acc && ch != 'i' && ch != 'o' && ch != 'l'
            });

        let cond3 = vec.windows(2).enumerate()
            .fold(0, |acc, (i, win)| {
                acc + if win[0] == win[1] {
                    if hmap.contains_key(win) {
                        if hmap[win] != i - 1 {
                            1
                        } else {
                            0
                        }
                    } else {
                        hmap.insert(win.to_vec(), i);
                        1
                    }
                } else {
                    0
                }
            }) >= 2;

        if cond1 && cond2 && cond3 {
            break;
        }
    }
    
    println!("The next password after that is {}.", password);
}