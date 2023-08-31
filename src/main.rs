use std::env;

#[path = "sha256/hash_finder.rs"]
mod hash_finder;

use crate::hash_finder::hash_finder::find_hash_with_zeroes;

fn main() {
    let args: Vec<String> = env::args().collect();

    let n: u32 = args[1].parse().unwrap();
    let f: u32 = args[2].parse().unwrap();

    find_hash_with_zeroes(n, f);
}