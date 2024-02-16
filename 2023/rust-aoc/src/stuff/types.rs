use std::collections::HashMap;
pub struct Node {
    pub value: Option<i32>,
    pub children: HashMap<char, Node>,
}

pub fn add_word(trie: &mut Node, word: &String, value: i32) {
    let mut cur: &mut Node = trie;

    for char in word.chars() {
        let letter = cur.children.get(&char);
        match letter {
            Some(_) => cur = cur.children.get_mut(&char).unwrap(),
            None => {
                let node = Node {
                    value: None,
                    children: HashMap::new(),
                };
                cur.children.insert(char, node);
                cur = cur.children.get_mut(&char).unwrap()
            }
        }
    }
    cur.value = Some(value);
}

pub fn search_word(trie: &Node, word: &String) -> Option<i32> {
    let mut cur = trie;
    
    for char in word.chars() {
        let letter = cur.children.get(&char);
        match letter {
            Some(a) => {cur = &a},
            None => {return None}
        }
    }
    return cur.value
}

pub fn build_dict(trie: &mut Node, words: &Vec<(String, i32)>) {
    for (word, value) in words {
        add_word(trie, word, value.clone());
    }
}
