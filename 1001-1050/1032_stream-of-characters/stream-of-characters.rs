
// 定义 Trie 节点

struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_word: bool,
}

// 实现 StreamChecker 结构体

struct StreamChecker {
    root: TrieNode,
    stream: Vec<char>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        // 创建 Trie 树

        let mut root = TrieNode {
            children: Default::default(),
            is_word: false,
        };
        for word in words {
            let mut node = &mut root;
            for ch in word.chars().rev() {
                let index = (ch as u8 - b'a') as usize;
                if node.children[index].is_none() {
                    node.children[index] = Some(Box::new(TrieNode {
                        children: Default::default(),
                        is_word: false,
                    }));
                }
                node = node.children[index].as_mut().unwrap();
            }
            node.is_word = true;
        }
        
        StreamChecker {
            root,
            stream: Vec::new(),
        }
    }
    
    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter);
        let mut node = &self.root;
        for i in (0..self.stream.len()).rev() {
            let ch = self.stream[i];
            let index = (ch as u8 - b'a') as usize;
            if let Some(child) = &node.children[index] {
                node = child;
                if node.is_word {
                    return true;
                }
            } else {
                return false;
            }
        }
        false

    }
}
