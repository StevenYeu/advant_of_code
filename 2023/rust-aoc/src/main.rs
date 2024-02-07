pub mod days;
pub mod stuff;
use std::fs::File;
use std::io::{Read, Result};
use std::collections::HashMap;
fn main() -> Result<()> {
    // let file_path = stuff::util::parse_args();
    // let mut file = File::open(file_path)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // let _ = days::day_one(&contents);
    run_trie();
    return Ok(());
}

fn run_trie() {
    let mut node = stuff::types::Node{value: None, children: HashMap::new() };
    let word = "one".to_string();
    stuff::types::add_word(&mut node, &word, 1)
}
