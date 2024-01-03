
struct TrieNode {
    is_end: bool,
    children: Vec<(char, TrieNode)>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_end: false,
            children: Vec::new(),
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;

        for c in word.chars() {
            if let Some(idx) = curr.children.iter().position(|e| e.0 == c) {
                curr = &mut curr.children[idx].1;
            } else {
                curr.children.push((c, TrieNode::new()));
                curr = &mut curr.children.last_mut().unwrap().1;
            }
        }

        curr.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut curr = &self.root;

        for c in word.chars() {
            if let Some(idx) = curr.children.iter().position(|e| e.0 == c) {
                curr = &curr.children[idx].1;
            } else {
                return false;
            }
        }

        curr.is_end

    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut curr = &self.root;

        for c in prefix.chars() {
            if let Some(idx) = curr.children.iter().position(|e| e.0 == c) {
                curr = &curr.children[idx].1;
            } else {
                return false;
            }
        }

        true

    }
}
