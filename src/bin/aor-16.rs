use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

#[derive(Clone)]
struct Aunt {
    id   : usize,
    items: HashMap<String, u32>
}

impl Aunt {
    fn new(id: usize, items: HashMap<String, u32>) -> Aunt {
        Aunt {
            id   : id,
            items: items
        }
    }

    fn equal(&self, a: Aunt) -> bool {
        for prop in self.items.keys() {
            if a.items.contains_key(prop) {
                if self.items[prop] != a.items[prop] {
                    return false;
                }
            }
        }
        return true;
    }

    fn equal_2(&self, a: Aunt) -> bool {
        for prop in self.items.keys() {
            if a.items.contains_key(prop) {
                if (prop == "cats" && self.items[prop] >= a.items[prop]) 
                || (prop == "trees" && self.items[prop] >= a.items[prop])
                || (prop == "pomeranians" && self.items[prop] <= a.items[prop]) 
                || (prop == "goldfish" && self.items[prop] <= a.items[prop]) 
                || (prop != "cats" && prop != "trees" && prop != "pomeranians" && prop != "goldfish" && self.items[prop] != a.items[prop]) {
                    return false;
                }
            }
        }
        return true;
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

    let mut ref_map = HashMap::new();
    ref_map.insert("children".to_string()   , 3);
    ref_map.insert("cats".to_string()       , 7);
    ref_map.insert("samoyeds".to_string()   , 2);
    ref_map.insert("pomeranians".to_string(), 3);
    ref_map.insert("akitas".to_string()     , 0);
    ref_map.insert("vizslas".to_string()    , 0);
    ref_map.insert("goldfish".to_string()   , 5);
    ref_map.insert("trees".to_string()      , 3);
    ref_map.insert("cars".to_string()       , 2);
    ref_map.insert("perfumes".to_string()   , 1);

    let ref_sue = Aunt::new(0, ref_map);

    let mut ret = 0;

    let mut aunts = Vec::new();

    for line in input.lines() {
        let mut parts: Vec<_> = line.split_whitespace().collect();

        let mut id_str = parts[1].to_string().clone(); id_str.pop();
        let id = id_str.parse().ok().expect("Could not parse into an integer!");

        let properties = parts.split_off(2);

        let mut map = HashMap::new();

        for elem in properties.chunks(2) {
            let mut prop = elem[0].to_string().clone(); prop.pop();
            let mut val_str = elem[1].to_string().clone();
            if let Some(ch) = val_str.pop() {
                if ch != ',' {
                   val_str.push(ch);
                }
            }
            let val = val_str.parse().ok().expect("Could not parse into an integer!");
            map.insert(prop.clone(), val);
        }

        let curr_aunt = Aunt::new(id, map);

        aunts.push(curr_aunt.clone());

        if ref_sue.equal(curr_aunt) {
            ret = id;
        }

    }

    println!("Sue #{} has sent the gift.", ret);

    for aunt in aunts.iter() {
        if ref_sue.equal_2(aunt.clone()) {
            ret = aunt.id;
            break;
        }
    }

    println!("Sue #{} has really sent the gift.", ret);
}