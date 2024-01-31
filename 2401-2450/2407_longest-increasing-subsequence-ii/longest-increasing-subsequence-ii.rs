
struct SegTree {
    n: usize,
    tree: Vec<i32>,
}

impl SegTree {
    fn new(arr: &[i32]) -> SegTree {
        let n = arr.len();
        let mut tree = vec![0; 2 * n];
        for i in 0..n {
            tree[i + n] = arr[i];
        }
        for i in (1..n).rev() {
            tree[i] = std::cmp::max(tree[i << 1], tree[i << 1 | 1]);
        }
        SegTree { n, tree }
    }

    fn query(&self, mut lo: usize, mut hi: usize) -> i32 {
        let mut ans = 0;
        lo += self.n;
        hi += self.n;
        while lo < hi {
            if lo & 1 == 1 {
                ans = std::cmp::max(ans, self.tree[lo]);
                lo += 1;
            }
            if hi & 1 == 1 {
                hi -= 1;
                ans = std::cmp::max(ans, self.tree[hi]);
            }
            lo >>= 1;
            hi >>= 1;
        }
        ans

    }

    fn update(&mut self, mut i: usize, val: i32) {
        i += self.n;
        self.tree[i] = val;
        while i > 1 {
            self.tree[i >> 1] = std::cmp::max(self.tree[i], self.tree[i ^ 1]);
            i >>= 1;
        }
    }
}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>, k: i32) -> i32 {
        let m = *nums.iter().max().unwrap();
        let mut ans = 0;
        let mut tree = SegTree::new(&vec![0; (m + 1) as usize]);
        for &x in nums.iter() {
            let val = tree.query(std::cmp::max(0, x - k) as usize, x as usize) + 1;
            ans = std::cmp::max(ans, val);
            tree.update(x as usize, val);
        }
        ans

    }
}
