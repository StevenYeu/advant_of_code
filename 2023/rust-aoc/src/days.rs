use std::collections::HashMap;
pub fn day_one(contents: &String) {
    let lines_iter = contents.split("\n").filter(|x| !x.is_empty());
    let mut ans: u32 = 0;
    for line in lines_iter {
        let mut num_map = line.chars().filter_map(|c| c.to_digit(10));
        let first = num_map.next().unwrap();
        let last = num_map.next_back().unwrap_or(first);
        ans += first * 10 + last;
    }
    print!("Total is {}", ans);
}

struct Node {
    value: Option<u32>,
    children: HashMap<char,Node> 
}

fn add_word(trie: &mut Node, word: &String) {
    for char in word.chars() {
        match trie.children.get(&char) {
            Some(a) => {
                
            }
            None => {
                trie.children.insert(char, Node{value: None, children: HashMap::new()});
            }
        }
    }
}

