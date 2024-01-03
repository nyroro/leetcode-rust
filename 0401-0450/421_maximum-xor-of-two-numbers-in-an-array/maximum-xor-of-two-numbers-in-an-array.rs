
struct Trie {
    children: [Option<Box<Trie>>; 2],
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: [None, None],
        }
    }

    fn insert(&mut self, num: i32) {
        let mut node = self;
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            node = node.children[bit as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }
    }

    fn find_max_xor(&self, num: i32) -> i32 {
        let mut result = 0;
        let mut node = self;
        for i in (0..32).rev() {
            let bit = (num >> i) & 1;
            if let Some(child) = &node.children[1 - bit as usize] {
                result |= 1 << i;
                node = child;
            } else {
                node = &node.children[bit as usize].as_ref().unwrap();
            }
        }
        result

    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut root = Trie::new();
        for &num in &nums {
            root.insert(num);
        }
        let mut max_xor = 0;
        for &num in &nums {
            max_xor = max_xor.max(root.find_max_xor(num));
        }
        max_xor

    }
}
