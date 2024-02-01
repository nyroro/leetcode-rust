
struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> FenwickTree {
        FenwickTree { tree: vec![0; size + 1] }
    }

    fn update(&mut self, mut index: i32, value: i32) {
        let mut index = index + 1;
        while index < self.tree.len() as i32 {
            self.tree[index as usize] += value;
            index += index & -index;
        }
    }

    fn query(&self, mut index: i32) -> i32 {
        let mut index = index + 1;
        let mut result = 0;
        while index > 0 {
            result += self.tree[index as usize];
            index -= index & -index;
        }
        result

    }
}

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        let mod_val = 1_000_000_007;
        let max_val = *instructions.iter().max().unwrap() as usize;
        let mut fenwick_tree = FenwickTree::new(max_val);
        let mut cost = 0;

        for (i, &num) in instructions.iter().enumerate() {
            let less = fenwick_tree.query(num - 1);
            let greater = i as i32 - fenwick_tree.query(num);
            cost = (cost + less.min(greater)) % mod_val;
            fenwick_tree.update(num, 1);
        }

        cost

    }
}
