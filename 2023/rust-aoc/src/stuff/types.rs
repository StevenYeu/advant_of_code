use std::collections::HashMap;
pub struct Node {
    pub value: Option<u32>,
    pub children: HashMap<char, Node>,
}

pub fn add_word(trie: &mut Node, word: &String, value: u32) {
    let mut cur: &mut Node = trie;

    for char in word.chars() {
        let letter = cur.children.get(&char);
        match letter {
            Some(a) => cur = cur.children.get_mut(&char).unwrap(),
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
