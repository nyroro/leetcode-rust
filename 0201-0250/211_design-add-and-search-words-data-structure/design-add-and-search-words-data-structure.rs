
struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    is_end: bool,
}

impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            children: [None; 26],
            is_end: false,
        }
    }
    
    fn add_word(&mut self, word: String) {
        let mut node = self;
        for &c in word.as_bytes() {
            let index = (c - b'a') as usize;
            node = node.children[index].get_or_insert_with(|| Box::new(WordDictionary {
                children: [None; 26],
                is_end: false,
            }));
        }
        node.is_end = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut stack = vec![(self, word.as_bytes())];
        while let Some((node, word)) = stack.pop() {
            if word.is_empty() {
                if node.is_end {
                    return true;
                }
            } else {
                match word[0] {
                    b'.' => {
                        for child in node.children.iter().flatten() {
                            stack.push((child, &word[1..]));
                        }
                    }
                    c => {
                        let index = (c - b'a') as usize;
                        if let Some(child) = &node.children[index] {
                            stack.push((child, &word[1..]));
                        }
                    }
                }
            }
        }
        false

    }
}
