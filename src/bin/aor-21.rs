use std::cmp::max;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

const INFTY: u32 = 1 << 30;

#[derive(Clone)]
struct Warrior {
    hp : u32,
    dmg: u32,
    arm: u32
}

impl Warrior {
    fn new(hp: u32, dmg: u32, arm: u32) -> Warrior {
        Warrior {
            hp : hp,
            dmg: dmg,
            arm: arm
        }
    }

    fn defeats(&self, w : Warrior) -> bool{ // self plays first
        let dmg_a = if self.dmg > w.arm {
            self.dmg - w.arm
        } else {
            1
        };
        let dmg_b = if w.dmg > self.arm {
            w.dmg - self.arm
        } else {
            1
        };
        let turns_a = if w.hp % dmg_a > 0 {
            w.hp / dmg_a + 1
        } else {
            w.hp / dmg_a
        };
        let turns_b = if self.hp % dmg_b > 0 {
            self.hp / dmg_b + 1
        } else {
            self.hp / dmg_b
        };
        turns_a <= turns_b
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

    let weapons = vec![(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armor = vec![(0, 0, 0), (13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
    let rings = vec![(25, 1, 0), (50, 2, 0), (100, 3, 0), (20, 0, 1), (40, 0, 2), (80, 0, 3)];

    let mut hp = 0;
    let mut dmg = 0;
    let mut arm = 0;
    let mut ret = INFTY;

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts[0] == "Hit" {
            hp = parts[2].parse().ok().expect("Could not parse into an integer!");
        } else if parts[0] == "Damage:" {
            dmg = parts[1].parse().ok().expect("Could not parse into an integer!");
        } else if parts[0] == "Armor:" {
            arm = parts[1].parse().ok().expect("Could not parse into an integer!");
        }
    }

    let opponent = Warrior::new(hp, dmg, arm);

    hp = 100;
    arm = 0;
    let mut curr;

    for wpn in weapons.iter() {
        let (cost, damage, _) = *wpn;
        if cost >= ret {
            continue;
        }
        if Warrior::new(hp, damage, arm).defeats(opponent.clone()) {
            ret = cost;
            continue;
        }
        curr = cost;
        dmg = damage;
        for armr in armor.iter() {
            let (cost, _, armor) = *armr;
            if curr + cost >= ret {
                continue;
            }
            if Warrior::new(hp, dmg, armor).defeats(opponent.clone()) {
                ret = curr + cost;
                continue;
            }
            curr += cost;
            arm = armor;
            for (i, ring_1) in rings.iter().enumerate() {
                let (cost, damage, armor) = *ring_1;
                if curr + cost >= ret {
                    continue;
                }
                if Warrior::new(hp, dmg + damage, arm + armor).defeats(opponent.clone()) {
                    ret = curr + cost;
                    continue;
                }
                curr += cost;
                dmg += damage;
                arm += armor;
                for ring_2 in rings[i + 1..].iter() {
                    let (cost, damage, armor) = *ring_2;
                        if curr + cost >= ret {
                        continue;
                    }
                    if Warrior::new(hp, dmg + damage, arm + armor).defeats(opponent.clone()) {
                        ret = curr + cost;
                        continue;
                    }
                }
                curr -= cost;
                dmg -= damage;
                arm -= armor;
            }
            curr -= cost;
            arm -= armor;
        }
    }

    println!("The minimal amount of gold needed is {}.", ret);

    ret = 0;
    arm = 0;

    for wpn in weapons.iter() {
        let (cost, damage, _) = *wpn;
        if !Warrior::new(hp, damage, arm).defeats(opponent.clone()) {
            ret = max(ret, cost);
        } else {
            continue;
        }
        curr = cost;
        dmg = damage;
        for armr in armor.iter() {
            let (cost, _, armor) = *armr;
            if !Warrior::new(hp, dmg, armor).defeats(opponent.clone()) {
                ret = max(ret, curr + cost);
            } else {
                continue;
            }
            curr += cost;
            arm = armor;
            for (i, ring_1) in rings.iter().enumerate() {
                let (cost, damage, armor) = *ring_1;
                if !Warrior::new(hp, dmg + damage, arm + armor).defeats(opponent.clone()) {
                    ret = max(ret, curr + cost);
                } else {
                    continue;
                }
                curr += cost;
                dmg += damage;
                arm += armor;
                for ring_2 in rings[i + 1..].iter() {
                    let (cost, damage, armor) = *ring_2;
                    if !Warrior::new(hp, dmg + damage, arm + armor).defeats(opponent.clone()) {
                        ret = max(ret, curr + cost);
                    }
                }
                curr -= cost;
                dmg -= damage;
                arm -= armor;
            }
            curr -= cost;
            arm -= armor;
        }
    }

    println!("The maximal amount of gold possible is {}.", ret);
}