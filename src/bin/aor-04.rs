extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let mut h = Md5::new();

    let key = "iwrupvqb".to_string();
    let mut ret = 1;

    loop {
        let input = key.clone() + &ret.to_string();
        h.input_str(&input[..]);
        let hash = h.result_str();
        if hash.starts_with("00000") {
            break;
        }
        ret = ret + 1;
        h.reset();
    }

    println!("The smallest positive integer is {}", ret);

    // Can pick up where we left off, as this time we need one more zero
    h.reset();

    loop {
        let input = key.clone() + &ret.to_string();
        h.input_str(&input[..]);
        let hash = h.result_str();
        if hash.starts_with("000000") {
            break;
        }
        ret = ret + 1;
        h.reset();
    }

    println!("The smallest positive integer is {}", ret);
}