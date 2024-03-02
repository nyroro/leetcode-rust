
struct SegmentTree {
    tree: Vec<i64>,
    lazy: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    fn new(size: usize) -> SegmentTree {
        SegmentTree {
            tree: vec![0; 4 * size],
            lazy: vec![0; 4 * size],
            n: size,
        }
    }

    fn propagate(&mut self, i: usize, j: usize, n: usize) {
        if self.lazy[n] == 0 {
            return;
        }
        let len = j - i + 1;
        if self.lazy[n] & 1 != 0 {
            self.tree[n] = len as i64 - self.tree[n];
        }
        if i != j {
            self.lazy[2 * n + 1] += self.lazy[n];
            self.lazy[2 * n + 2] += self.lazy[n];
        }
        self.lazy[n] = 0;
    }

    fn update(&mut self, s: usize, e: usize, i: usize, j: usize, n: usize) {
        self.propagate(i, j, n);
        if s > j || e < i || i > j {
            return;
        }
        if s <= i && e >= j {
            self.lazy[n] += 1;
            self.propagate(i, j, n);
            return;
        }
        let mid = (j - i) / 2 + i;
        self.update(s, e, i, mid, 2 * n + 1);
        self.update(s, e, mid + 1, j, 2 * n + 2);
        self.tree[n] = self.tree[2 * n + 1] + self.tree[2 * n + 2];
    }

    fn query(&mut self, l: usize, r: usize, i: usize, j: usize, n: usize) -> i64 {
        self.propagate(i, j, n);
        if r < i || l > j {
            0

        } else if l <= i && r >= j {
            self.tree[n]
        } else {
            let mid = (j - i) / 2 + i;
            self.query(l, r, i, mid, 2 * n + 1) + self.query(l, r, mid + 1, j, 2 * n + 2)
        }
    }
}



impl Solution {
    pub fn handle_query(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = nums1.len();
        let mut tree = SegmentTree::new(n);
        let mut sum: i64 = nums2.iter().map(|&x| x as i64).sum();

        for (i, &num) in nums1.iter().enumerate() {
            if num == 1 {
                tree.update(i, i, 0, n - 1, 0);
            }
        }

        let mut result = Vec::new();
        for query in queries {
            match query[0] {
                1 => {
                    tree.update(query[1] as usize, query[2] as usize, 0, n - 1, 0);
                }
                2 => {
                    sum += tree.query(0, n - 1, 0, n - 1, 0) * query[1] as i64;
                }
                3 => {
                    result.push(sum);
                }
                _ => {}
            }
        }

        result

    }
}
