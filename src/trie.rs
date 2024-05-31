
#[derive(Default, Debug)]
struct TrieNode {
    children: Vec<TrieNode>,
    id: u16
}


impl TrieNode {
    fn new() -> Self {
        let mut trinode = TrieNode {
            children: Vec::new(),
            id: 0
        };
        for i in 0..256 {
            trinode.children[i] = TrieNode::new();
        }
        trinode
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
        node.id = id
    }

    fn search_the_longest(&self, word: &str) -> (u16, u16) {
        let mut node = &self.root;
        let mut old_node: &TrieNode = &self.root;
        let mut index = 0;
        let mut old_index = 0;
        for ch in word.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                if node.id != 0 {
                    old_node = node;
                    old_index = index;
                }
                node = next_node;
                index += 1;
            } else {
                if node.id == 0 {
                    return (old_index, old_node.id);
                }
                else {
                    return (index, node.id);
                }
            }
        }
        if node.id == 0 {
            return (old_index, old_node.id);
        }
        else {
            return (index, node.id);
        }
    }

    pub(crate) fn tokenize(&self, text: &str) -> Vec<u16> {
        let mut vec: Vec<u16> = Vec::new();
        let mut index: usize = 0;
        let original_string = String::from(text);
        let char_end = original_string.chars().count();
        loop {
            let start_byte = original_string.char_indices().nth(index).map(|(i, _)| i).unwrap_or(0);
            let text = &original_string[start_byte..];
            let result = self.search_the_longest(&text);
            if result.0 != 0 {
                vec.push(result.1.into());
                index += <u16 as Into<usize>>::into(result.0);
            } else {
                return vec;
            }
            if index >= char_end {
                return vec;
            }
        }
    }
}