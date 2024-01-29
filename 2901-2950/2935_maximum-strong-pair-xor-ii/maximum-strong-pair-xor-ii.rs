
struct TrieNode {
    children: [Option<Box<TrieNode>>; 2],
    count: i32,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: [None, None],
            count: 0,
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

    fn insert(&mut self, value: i32) {
        let mut curr_node = &mut self.root;
        for i in (0..20).rev() {
            let bit = (value >> i) & 1;
            if curr_node.children[bit as usize].is_none() {
                curr_node.children[bit as usize] = Some(Box::new(TrieNode::new()));
            }
            curr_node = curr_node.children[bit as usize].as_mut().unwrap();
            curr_node.count += 1;
        }
    }

    fn delete(&mut self, value: i32) {
        let mut curr_node = &mut self.root;
        for i in (0..20).rev() {
            let bit = (value >> i) & 1;
            curr_node = curr_node.children[bit as usize].as_mut().unwrap();
            curr_node.count -= 1;
        }
    }

    fn find_max(&self, value: i32) -> i32 {
        let mut curr_node = &self.root;
        let mut result = 0;
        for i in (0..20).rev() {
            let bit = (value >> i) & 1;
            let complement = &curr_node.children[1 - (bit as usize)];
            if let Some(node) = complement {
                if node.count > 0 {
                    curr_node = node;
                    result = (result << 1) | 1;
                } else {
                    curr_node = curr_node.children[bit as usize].as_ref().unwrap();
                    result = result << 1;
                }
            } else {
                curr_node = curr_node.children[bit as usize].as_ref().unwrap();
                result = result << 1;
            }
        }
        result

    }
}



impl Solution {
    pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut i = 0;
        let mut trie = Trie::new();
        let mut result = 0;
        for j in 0..nums.len() {
            trie.insert(nums[j]);
            while i < j && nums[j] - nums[i] > nums[i] {
                trie.delete(nums[i]);
                i += 1;
            }
            result = result.max(trie.find_max(nums[j]));
        }
        result

    }
}
