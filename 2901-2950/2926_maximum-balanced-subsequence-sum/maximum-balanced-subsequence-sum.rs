
impl Solution {
    pub fn max_balanced_subsequence_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut arr: Vec<i32> = Vec::with_capacity(n);
        for i in 0..n {
            arr.push(nums[i] - i as i32);
        }
        arr.sort();
        arr.dedup();

        let m = arr.len();
        let mut tree = BinaryIndexedTree::new(m);
        for i in 0..n {
            let j = Solution::search(&arr, nums[i] - i as i32) + 1;
            let v = i64::max(tree.query(j), 0) + nums[i] as i64;
            tree.update(j, v);
        }
        tree.query(m) as i64

    }

    fn search(nums: &Vec<i32>, x: i32) -> usize {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] >= x {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l

    }
}

struct BinaryIndexedTree {
    n: usize,
    c: Vec<i64>,
}

impl BinaryIndexedTree {
    fn new(n: usize) -> BinaryIndexedTree {
        BinaryIndexedTree {
            n,
            c: vec![-1e18 as i64; n + 1],
        }
    }

    fn update(&mut self, mut x: usize, v: i64) {
        while x <= self.n {
            self.c[x] = self.c[x].max(v);
            x += x & (!x + 1);
        }
    }

    fn query(&self, mut x: usize) -> i64 {
        let mut mx = -1e18 as i64;
        while x > 0 {
            mx = mx.max(self.c[x]);
            x -= x & (!x + 1);
        }
        mx

    }
}
