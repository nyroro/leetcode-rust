
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    count: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            count: 0,
        }
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut root = TrieNode::new();
        let mut result = Vec::new();

        for word in words.iter() {
            let mut node = &mut root;
            for &ch in word.as_bytes() {
                let idx = (ch - b'a') as usize;
                node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
                node.count += 1;
            }
        }

        for word in words.iter() {
            let mut node = &root;
            let mut score = 0;
            for &ch in word.as_bytes() {
                let idx = (ch - b'a') as usize;
                if let Some(next_node) = &node.children[idx] {
                    node = next_node;
                    score += node.count;
                } else {
                    break;
                }
            }
            result.push(score);
        }

        result

    }
}
