
struct TrieNode {
    children: Vec<Option<Box<TrieNode>>>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: vec![None, None],
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

    fn insert(&mut self, num: i32) {
        let mut node = &mut self.root;
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            node = node.children[bit as usize].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
    }

    fn find_max_xor(&self, num: i32) -> i32 {
        let mut node = &self.root;
        let mut result = 0;
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            let complement = 1 - bit;
            if let Some(next_node) = &node.children[complement as usize] {
                result |= 1 << i;
                node = next_node;
            } else {
                node = &node.children[bit as usize].as_ref().unwrap();
            }
        }
        result

    }
}

impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut queries: Vec<(i32, i32, usize)> = queries

            .into_iter()
            .enumerate()
            .map(|(i, q)| (q[0], q[1], i))
            .collect();
        queries.sort_unstable_by_key(|q| q.1);

        let mut trie = Trie::new();
        let mut result = vec![-1; queries.len()];
        let mut j = 0;
        for (x, m, i) in queries {
            while j < nums.len() && nums[j] <= m {
                trie.insert(nums[j]);
                j += 1;
            }
            if j > 0 {
                result[i] = trie.find_max_xor(x);
            }
        }
        result

    }
}
