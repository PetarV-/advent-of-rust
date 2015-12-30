use std::cmp::min;

fn main() {
    let lim = 36000000;

    let mut houses = vec![0; lim/10];
    let mut ret = 0;

    for i in 1..lim/10 {
        for j in (1..lim/(10 * i)) {
            houses[j * i] += i * 10;
        }
        if houses[i] >= lim {
            ret = i;
            break;
        }
    }

    println!("The first house to receive sufficient presents is house {}.", ret);

    for i in 1..lim/10 {
        houses[i] = 0;
    }

    for i in 1..lim/10 {
        for j in (1..min(50, lim/(10 * i))) {
            houses[j * i] += i * 11;
        }
        if houses[i] >= lim {
            ret = i;
            break;
        }
    }

    println!("The first house to now receive sufficient presents is house {}.", ret);
}