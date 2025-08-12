#[derive(Default, Debug)]
struct TrieNode {
    children: [[Option<Box<TrieNode>>; 16]; 16],
    id: Option<u16>,
}


impl TrieNode {
    fn new() -> Self {
        let mut trinode = TrieNode {
            children: Default::default(),
            id: None,
        };
        for index in 0..256 {
            trinode.children[index >> 4][index & 15] = None;
        }
        trinode
    }
}

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
}


impl Trie {
    pub(crate) fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    /// Insert a word (as bytes) and its id into the trie.
    pub(crate) fn insert(&mut self, word: &[u8], id: u16) {
        let mut node = &mut self.root;
        for &ch in word {
            let ch = u8::from_be(ch) as usize;
            let index_a = ch >> 4;
            let index_b = ch & 15;
            if node.children[index_a][index_b].is_none() {
                node.children[index_a][index_b] = Option::from(Box::new(TrieNode::new()));
            }
            match &mut node.children[index_a][index_b] {
                Some(next_node) => node = next_node,
                None => unreachable!(),
            }
        }
        node.id = Some(id);
    }

    fn search_the_longest(&self, word: &[u8]) -> (u16, u16) {
        let mut node = &self.root;
        let mut old_node: &TrieNode = &self.root;
        let mut index = 0;
        let mut old_index = 0;
        for &ch in word {
            let ch = u8::from_be(ch) as usize;
            let index_a = ch >> 4;
            let index_b = ch & 15;
            if let Some(next_node) = &node.children[index_a][index_b]{
                if let Some(_id) = node.id {
                    old_node = node;
                    old_index = index;
                }
                node = &next_node;
                index += 1;
            } else {
                if node.id.is_none() {
                    return (old_index, old_node.id.unwrap_or(0));
                } else {
                    return (index, node.id.unwrap());
                }
            }
        }
        if node.id.is_none() {
            return (old_index, old_node.id.unwrap_or(0));
        } else {
            return (index, node.id.unwrap());
        }
    }

    /// Tokenize a string into a vector of token IDs.
    pub(crate) fn tokenize(&self, text: &str) -> Vec<u16> {
        let mut vec: Vec<u16> = Vec::new();
        let text_bytes = text.as_bytes();
        let text_length = text_bytes.len();
        let mut index: usize = 0;
        loop {
            let result = self.search_the_longest(&text_bytes[index..]);
            if result.0 != 0 {
                vec.push(result.1.into());
                index += <u16 as Into<usize>>::into(result.0);
            } else {
                return vec;
            }
            if index >= text_length {
                return vec;
            }
        }
    }
}