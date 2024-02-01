
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: [None; 26],
            is_end: false,
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

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for &c in word.as_bytes() {
            let index = (c - b'a') as usize;
            node = node.children[index].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    fn find_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut node = &self.root;
        for &c in prefix.as_bytes() {
            let index = (c - b'a') as usize;
            if let Some(next) = &node.children[index] {
                node = next;
            } else {
                return result;
            }
        }
        self.dfs(&mut result, &mut String::from(prefix), &node);
        result

    }

    fn dfs(&self, result: &mut Vec<String>, mut current: &mut String, node: &TrieNode) {
        if result.len() == 3 {
            return;
        }
        if node.is_end {
            result.push(current.clone());
        }
        for (i, child) in node.children.iter().enumerate() {
            if let Some(next) = child {
                current.push((b'a' + i as u8) as char);
                self.dfs(result, &mut current, next);
                current.pop();
            }
        }
    }
}

impl Solution {
    pub fn suggested_products(products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for product in &products {
            trie.insert(product);
        }
        let mut result = Vec::new();
        let mut prefix = String::new();
        for c in search_word.chars() {
            prefix.push(c);
            result.push(trie.find_with_prefix(&prefix));
        }
        result

    }
}
