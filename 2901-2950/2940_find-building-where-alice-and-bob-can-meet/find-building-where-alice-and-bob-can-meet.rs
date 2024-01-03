
// 定义一个结构体BinaryIndexedTree

struct BinaryIndexedTree {
    n: usize,
    c: Vec<i32>,
}

impl BinaryIndexedTree {
    // 实现BinaryIndexedTree的new方法

    fn new(n: usize) -> BinaryIndexedTree {
        BinaryIndexedTree {
            n,
            c: vec![-1; n + 1],
        }
    }

    // 实现BinaryIndexedTree的update方法

    fn update(&mut self, mut x: usize, v: i32) {
        while x <= self.n {
            if self.c[x] == -1 || v < self.c[x] {
                self.c[x] = v;
            }
            x += (x as i32 & -(x as i32)) as usize;
        }
    }

    // 实现BinaryIndexedTree的query方法

    fn query(&self, mut x: usize) -> i32 {
        let mut mi = i32::max_value();
        while x > 0 {
            if self.c[x] != -1 {
                mi = mi.min(self.c[x]);
            }
            x -= (x as i32 & -(x as i32)) as usize;
        }
        if mi == i32::max_value() {
            -1

        } else {
            mi

        }
    }
}

// 实现Solution结构体



impl Solution {
    // 实现leftmost_building_queries方法

    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = heights.len();
        let m = queries.len();
        let mut queries = queries;
        for i in 0..m {
            if queries[i][0] > queries[i][1] {
                queries[i].swap(0, 1);
            }
        }
        let mut idx: Vec<usize> = (0..m).collect();
        idx.sort_by(|&i, &j| queries[j][1].cmp(&queries[i][1]));

        let mut s = heights.clone();
        s.sort();
        s.dedup();

        let mut ans = vec![0; m];
        let mut j = n - 1;
        let mut tree = BinaryIndexedTree::new(n);

        for i in idx {
            let l = queries[i][0] as usize;
            let r = queries[i][1] as usize;
            while j > r {
                let k = n - s.binary_search(&heights[j]).unwrap() + 1;
                tree.update(k, j as i32);
                j -= 1;
            }
            if l == r || heights[l] < heights[r] {
                ans[i] = r as i32;
            } else {
                let k = n - s.binary_search(&heights[l]).unwrap();
                ans[i] = tree.query(k) as i32;
            }
        }
        ans

    }
}
