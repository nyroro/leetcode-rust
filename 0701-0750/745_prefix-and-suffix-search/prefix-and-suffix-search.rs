
use std::collections::HashMap;

struct TrieNode {
    children: HashMap<char, TrieNode>,
    weight: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            weight: -1,
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie { root: TrieNode::new() }
    }

    fn insert(&mut self, word: &str, weight: i32) {
        let mut node = &mut self.root;
        node.weight = weight;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(TrieNode::new());
            node.weight = weight;
        }
    }

    fn search(&self, word: &str) -> i32 {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(n) = node.children.get(&c) {
                node = n;
            } else {
                return -1;
            }
        }
        node.weight

    }
}

struct WordFilter {
    trie: Trie,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for (i, word) in words.iter().enumerate() {
            for j in 0..word.len() {
                let suffix = format!("{}#{}", &word[j..], word);
                trie.insert(&suffix, i as i32);
            }
        }
        WordFilter { trie }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let word = format!("{}#{}", suff, pref);
        self.trie.search(&word)
    }
}
