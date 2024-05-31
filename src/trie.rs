#[derive(Default, Debug)]
struct TrieNode {
    children: Vec<Option<TrieNode>>,
    id: u16
}


impl TrieNode {
    fn new() -> Self {
        let mut trinode = TrieNode {
            children: Vec::new(),
            id: 0
        };
        for _ in 0..256 {
            trinode.children.push(None);
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

    pub(crate) fn insert(&mut self, word: &str, id: u16) {
        let mut node = &mut self.root;
        for ch in word.bytes() {
            if node.children[ch as usize].is_none() {
                node.children[ch as usize] = Option::from(TrieNode::new());
            }
            match &mut node.children[ch as usize] {
                Some(next_node) => node = next_node,
                None => unreachable!(),  // We've just checked that it's not None
            }
        }
        node.id = id
    }

    fn search_the_longest(&self, word: &str) -> (u16, u16) {
        let mut node = &self.root;
        let mut old_node: &TrieNode = &self.root;
        let mut index = 0;
        let mut old_index = 0;
        for ch in word.bytes() {
            if let Some(next_node) = &node.children[ch as usize] {
                if node.id != 0 {
                    old_node = node;
                    old_index = index;
                }
                node = &next_node;
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
        let text_length = text.len();
        let mut index: usize = 0;
        loop {
            let result = self.search_the_longest(&text[index..]);
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