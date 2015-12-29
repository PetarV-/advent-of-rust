use std::io::prelude::*;

enum State {
    Tracking(char, usize)
}

fn main() {
    let mut input = "1321131112".to_string();

    for _ in 0..40 {
        let (acc, State::Tracking(c, l)) = input.chars()
            .fold(("".to_string(), State::Tracking('\0', 0)), |(acc, state), ch| {
                let State::Tracking(x, len) = state;
                if ch == x {
                    (acc, State::Tracking(x, len + 1))
                } else if len > 0  {
                    (acc + &len.to_string() + &x.to_string(), State::Tracking(ch, 1))
                } else {
                    (acc, State::Tracking(ch, 1))
                }
            });
        input = acc.to_string() + &l.to_string() + &c.to_string();
    }

    let ret = input.len();

    println!("The final length of the sequence after 40 steps is {}.", ret);

    for _ in 40..50 {
        let (acc, State::Tracking(c, l)) = input.chars()
            .fold(("".to_string(), State::Tracking('\0', 0)), |(acc, state), ch| {
                let State::Tracking(x, len) = state;
                if ch == x {
                    (acc, State::Tracking(x, len + 1))
                } else if len > 0  {
                    (acc + &len.to_string() + &x.to_string(), State::Tracking(ch, 1))
                } else {
                    (acc, State::Tracking(ch, 1))
                }
            });
        input = acc.to_string() + &l.to_string() + &c.to_string();
    }

    let ret = input.len();

    println!("The final length of the sequence after 50 steps is {}.", ret);
}