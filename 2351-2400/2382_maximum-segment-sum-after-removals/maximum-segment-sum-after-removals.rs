
struct UnionFind {
    root: Vec<i32>,
    sum: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let root = vec![-1; n];
        let sum = vec![0; n];
        UnionFind { root, sum }
    }

    fn insert(&mut self, x: usize, value: i32) {
        if self.root[x] != -1 || self.sum[x] != 0 {
            return;
        }
        self.root[x] = x as i32;
        self.sum[x] = value as i64;
    }

    fn find(&mut self, x: usize) -> usize {
        let mut x = x as i32;
        while self.root[x as usize] != x {
            let fa = self.root[x as usize];
            let ga = self.root[fa as usize];
            self.root[x as usize] = ga;
            x = fa;
        }
        x as usize

    }

    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        self.root[rx] = ry as i32;
        self.sum[ry] += self.sum[rx];
    }

    fn has(&self, x: usize) -> bool {
        self.root[x] != -1 || self.sum[x] != 0

    }
}

impl Solution {
    pub fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64> {
        let n = remove_queries.len();
        let mut ret = vec![0; n];
        let mut max = 0;
        let mut uf = UnionFind::new(n);

        for i in (0..n).rev() {
            let u = remove_queries[i] as usize;
            uf.insert(u, nums[u]);
            for v in &[u - 1, u + 1] {
                if *v < n && uf.has(*v) {
                    uf.union(*v, u);
                }
            }
            ret[i] = max;
            let ru = uf.find(u);
            max = max.max(uf.sum[ru]);
        }

        ret

    }
}
