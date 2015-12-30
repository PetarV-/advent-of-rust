use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

const INFTY: i32 = 1 << 30;

#[derive(Clone, Hash, PartialEq, Eq)]
struct Wizard {
    hp  : i32,
    arm : i32,
    mana: i32
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Warrior {
    hp : i32,
    dmg: i32
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    shield  : u8,
    poison  : u8,
    recharge: u8
}

impl Wizard {
    fn new(hp: i32, arm: i32, mana: i32) -> Wizard {
        Wizard {
            hp  : hp,
            arm : arm,
            mana: mana
        }
    }


    fn can_cast_spell(&self, st: State, name: &str, cost: i32) -> bool {
        self.mana >= cost 
        && (name != "shld"  || st.shield   == 0)
        && (name != "psn"   || st.poison   == 0)
        && (name != "rchrg" || st.recharge == 0)
    }
}

impl Warrior {
    fn new(hp: i32, dmg: i32) -> Warrior {
        Warrior {
            hp : hp,
            dmg: dmg
        }
    }
}

impl State {
    fn new(shield: u8, poison: u8, recharge: u8) -> State {
        State {
            shield  : shield,
            poison  : poison,
            recharge: recharge
        }
    }
}

fn cripple(wiz: Wizard, turn: bool, hard: bool) -> (Wizard, bool) {
    if !hard || !turn {
        (wiz, false)
    } else {
        let next_hp = wiz.hp - 1;
        (Wizard::new(next_hp, wiz.arm, wiz.mana),
         next_hp <= 0)
    }
}

fn apply_effects(wiz: Wizard, war: Warrior, st: State) -> (Wizard, Warrior, State, bool) {
    let next_shield;
    let next_arm;
    if st.shield > 0 {
        next_arm = 7;
        next_shield = st.shield - 1;
    } else {
        next_arm = 0;
        next_shield = 0;
    }
    let next_poison;
    let next_war_hp;
    if st.poison > 0 {
        next_war_hp = war.hp - 3;
        next_poison = st.poison - 1;
    } else {
        next_war_hp = war.hp;
        next_poison = 0;
    }
    let next_recharge;
    let next_mana;
    if st.recharge > 0 {
        next_mana = wiz.mana + 101;
        next_recharge = st.recharge - 1;
    } else {
        next_mana = wiz.mana;
        next_recharge = 0;
    }
    (Wizard::new(wiz.hp, next_arm, next_mana), 
     Warrior::new(next_war_hp, war.dmg), 
     State::new(next_shield, next_poison, next_recharge),
     next_war_hp <= 0)
}

fn cast_spell(wiz: Wizard, war: Warrior, st: State, name: &str, cost: i32) -> (Wizard, Warrior, State, bool) {
    let next_mana = wiz.mana - cost;
    if name == "mssl" {
        let next_war_hp = war.hp - 4;
        (Wizard::new(wiz.hp, wiz.arm, next_mana),
         Warrior::new(next_war_hp, war.dmg),
         st,
         next_war_hp <= 0)
    } else if name == "drn" {
        let next_war_hp = war.hp - 2;
        let next_wiz_hp = wiz.hp + 2;
        (Wizard::new(next_wiz_hp, wiz.arm, next_mana),
         Warrior::new(next_war_hp, war.dmg),
         st,
         next_war_hp <= 0)
    } else if name == "shld" {
        (Wizard::new(wiz.hp, wiz.arm, next_mana),
         war,
         State::new(6, st.poison, st.recharge),
         false)
    } else if name == "psn" {
        (Wizard::new(wiz.hp, wiz.arm, next_mana),
         war,
         State::new(st.shield, 6, st.recharge),
         false)
    } else if name == "rchrg" {
        (Wizard::new(wiz.hp, wiz.arm, next_mana),
         war,
         State::new(st.shield, st.poison, 5),
         false)
    } else {
        panic!("Tried to cast a non-existent spell!");
    }
}

fn battle(wiz: Wizard, war: Warrior, st: State, wizard_turn: bool, hard: bool, mana_spent: i32, spells: Vec<(i32, &str)>, memo: &mut HashMap<(Wizard, Warrior, State, bool), i32>, ret: &mut i32) {
    if mana_spent >= *ret {
        return;
    }
    let key = (wiz.clone(), war.clone(), st.clone(), wizard_turn);
    if memo.contains_key(&key) && memo[&key] <= mana_spent {
        return;
    } else {
        memo.insert(key, mana_spent);
    }

    let (next_wiz, done) = cripple(wiz, wizard_turn, hard);
    if done {
        return;
    }
    else {
        let (next_wiz, next_war, next_st, done) = apply_effects(next_wiz, war, st);
        if done {
            *ret = mana_spent;
        } else if wizard_turn {
            for spell in spells.iter() {
                let (cost, name) = *spell;
                if mana_spent + cost >= *ret {
                    continue;
                }
                if next_wiz.can_cast_spell(next_st.clone(), name, cost) {
                    let (nexter_wiz, nexter_war, nexter_st, done) = cast_spell(next_wiz.clone(), next_war.clone(), next_st.clone(), name, cost);
                    if done {
                        *ret = mana_spent + cost;
                    } else {
                        battle(nexter_wiz, 
                               nexter_war, 
                               nexter_st, 
                               false, 
                               hard,
                               mana_spent + cost, 
                               spells.clone(), 
                               memo,
                               ret);
                    }
                }
            }
        } else {
            let dmg_done = if next_war.dmg > next_wiz.arm {
                next_war.dmg - next_wiz.arm
            } else {
                1
            };
            if next_wiz.hp <= dmg_done {
                return;
            } else {
                let next_wiz_hp = next_wiz.hp - dmg_done;
                battle(Wizard::new(next_wiz_hp, next_wiz.arm, next_wiz.mana), 
                       next_war.clone(), 
                       next_st,
                       true,
                       hard,
                       mana_spent,
                       spells,
                       memo,
                       ret);
            }
        }
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

    let spells = vec![(53, "mssl"), (73, "drn"), (113, "shld"), (173, "psn"), (229, "rchrg")];

    let mut hp = 0;
    let mut dmg = 0;
    let mut ret = INFTY;

    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts[0] == "Hit" {
            hp = parts[2].parse().ok().expect("Could not parse into an integer!");
        } else if parts[0] == "Damage:" {
            dmg = parts[1].parse().ok().expect("Could not parse into an integer!");
        }
    }

    let player = Wizard::new(50, 0, 500);
    let opponent = Warrior::new(hp, dmg);
    let state = State::new(0, 0, 0);
    let mut memo: HashMap<(Wizard, Warrior, State, bool), i32> = HashMap::new();

    battle(player.clone(), opponent.clone(), state.clone(), true, false, 0, spells.clone(), &mut memo, &mut ret);

    println!("The optimal amount of mana spent is {}.", ret);

    ret = INFTY;
    memo.clear();

    battle(player, opponent, state, true, true, 0, spells, &mut memo, &mut ret);

    println!("The optimal amount of mana spent on hard mode is {}.", ret);
}