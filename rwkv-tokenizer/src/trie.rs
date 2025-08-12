#[derive(Default, Debug)]
struct TrieNode {
    children: [[Option<Box<TrieNode>>; 16]; 16],
    id: Option<u16>,
}


impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            id: None,
        }
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
            let ch = ch as usize;
            let index_a = ch >> 4;
            let index_b = ch & 15;
            if node.children[index_a][index_b].is_none() {
                node.children[index_a][index_b] = Some(Box::new(TrieNode::new()));
            }
            node = node.children[index_a][index_b].as_mut().unwrap();
        }
        node.id = Some(id);
    }

    fn search_the_longest(&self, word: &[u8]) -> (u16, u16) {
        let mut node = &self.root;
        let mut old_node: &TrieNode = &self.root;
        let mut index = 0;
        let mut old_index = 0;
        for &ch in word {
            let ch = ch as usize;
            let index_a = ch >> 4;
            let index_b = ch & 15;
            if let Some(next_node) = &node.children[index_a][index_b]{
                if node.id.is_some() {
                    old_node = node;
                    old_index = index;
                }
                node = next_node;
                index += 1;
            } else {
                return if node.id.is_none() {
                    (old_index, old_node.id.unwrap_or(0))
                } else {
                    (index, node.id.unwrap())
                };
            }
        }
        if node.id.is_none() {
            (old_index, old_node.id.unwrap_or(0))
        } else {
            (index, node.id.unwrap())
        }
    }

    /// Tokenize a string into a vector of token IDs.
    pub(crate) fn tokenize(&self, text: &str) -> Vec<u16> {
        let mut tokens = Vec::new();
        let text_bytes = text.as_bytes();
        let mut index = 0;
        
        while index < text_bytes.len() {
            let (length, token_id) = self.search_the_longest(&text_bytes[index..]);
            if length == 0 {
                break;
            }
            tokens.push(token_id);
            index += length as usize;
        }
        
        tokens
    }
}