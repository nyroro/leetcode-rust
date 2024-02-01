
use std::collections::HashMap;

struct WordDictionary {
    children: HashMap<u8, WordDictionary>,
    is_end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            children: HashMap::new(),
            is_end: false,
        }
    }
    
    fn add_word(&mut self, word: String) {
        let mut node = self;
        for &c in word.as_bytes() {
            node = node.children.entry(c).or_insert(WordDictionary {
                children: HashMap::new(),
                is_end: false,
            });
        }
        node.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut stack = vec![self];
        let word_bytes = word.as_bytes();
        for &c in word_bytes {
            let mut next_stack = Vec::new();
            for node in &stack {
                if c == b'.' {
                    for child in node.children.values() {
                        next_stack.push(child);
                    }
                } else if let Some(child) = node.children.get(&c) {
                    next_stack.push(child);
                }
            }
            stack = next_stack;
        }
        stack.iter().any(|node| node.is_end)
    }
}
