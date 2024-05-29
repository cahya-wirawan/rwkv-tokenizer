use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
    id: u16
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_end_of_word: false,
            id: 0
        }
    }
}

#[derive(Debug)]
pub(crate) struct Trie {
    root: TrieNode,
}

impl Trie {
    pub(crate) fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    pub(crate) fn insert(&mut self, word: String, id: u16) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_end_of_word = true;
        node.id = id
    }

    fn search_the_longest(&self, word: &str) -> (u16, u16) {
        let mut node = &self.root;
        let mut index = 0;
        for ch in word.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
                index += 1;
            } else {
                return (index, node.id);
            }
        }
        (index, node.id)
    }

    pub(crate) fn tokenize(&self, text: &str) -> Vec<u16> {
        let mut vec: Vec<u16> = Vec::new();
        let mut index: usize = 0;
        loop {
            let result = self.search_the_longest(&text[index..]);
            if result.0 != 0 {
                vec.push(result.1.into());
                index += <u16 as Into<usize>>::into(result.0);
            } else {
                return vec;
            }
        }
    }
}