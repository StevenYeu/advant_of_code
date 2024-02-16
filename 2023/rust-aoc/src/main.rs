pub mod days;
pub mod stuff;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result};
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
    let mut node = stuff::types::Node {
        value: None,
        children: HashMap::new(),
    };
    let word = "bad".to_string();
    let words: Vec<(String, i32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .map(|(x,y)| (x.to_string(), *y))
    .collect();
    stuff::types::build_dict(&mut node, &words);
    let value = stuff::types::search_word(&node, &word);
    match value {
        Some(a) => {
            println!("{}", a)
        }
        None => {
            println!("No value")
        }
    }
}
