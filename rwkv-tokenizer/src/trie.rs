#[derive(Default, Debug)]
struct TrieNode {
    children: [[Option<Box<TrieNode>>; 16]; 16],
    id: u16
}


impl TrieNode {
    fn new() -> Self {
        let mut trinode = TrieNode {
            children: Default::default(),
            id: 0
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

    pub(crate) fn insert(&mut self, word: &Vec<u8>, id: u16) {
        let mut node = &mut self.root;
        for ch in word {
            let ch = u8::from_be(*ch) as usize;
            let index_a = ch >> 4;
            let index_b = ch & 15;
            if node.children[index_a][index_b].is_none() {
                node.children[index_a][index_b] = Option::from(Box::new(TrieNode::new()));
            }
            match &mut node.children[index_a][index_b] {
                Some(next_node) => node = next_node,
                None => unreachable!(),  // We've just checked that it's not None
            }
        }
        node.id = id
    }

    /// Greedy longest match starting at `bytes[from..]`: walks the trie until it dead-ends,
    /// remembering the deepest node that terminated a valid token. Returns the matched byte
    /// length and its token id, or `(0, 0)` when nothing matches.
    #[inline]
    pub(crate) fn longest_match(&self, bytes: &[u8], from: usize) -> (usize, u16) {
        let mut node = &self.root;
        let mut best_length = 0;
        let mut best_id = 0;
        for (offset, ch) in bytes[from..].iter().enumerate() {
            let ch = *ch as usize;
            match &node.children[ch >> 4][ch & 15] {
                Some(next_node) => node = next_node,
                None => break,
            }
            if node.id != 0 {
                best_length = offset + 1;
                best_id = node.id;
            }
        }
        (best_length, best_id)
    }

    /// Tokenize `bytes` from `from` until at least `until`, appending ids to `out`.
    ///
    /// Returns the position where tokenization actually stopped. That can exceed `until`
    /// when the final token straddles the limit, which is what lets `encode_parallel`
    /// detect a mis-guessed chunk boundary.
    pub(crate) fn tokenize_range(&self, bytes: &[u8], from: usize, until: usize, out: &mut Vec<u16>) -> usize {
        let mut index = from;
        while index < until {
            let (length, id) = self.longest_match(bytes, index);
            if length == 0 {
                break;
            }
            out.push(id);
            index += length;
        }
        index
    }

    pub(crate) fn tokenize(&self, text: &str) -> Vec<u16> {
        let bytes = text.as_bytes();
        let mut vec: Vec<u16> = Vec::with_capacity(bytes.len() / 3 + 1);
        self.tokenize_range(bytes, 0, bytes.len(), &mut vec);
        vec
    }
}