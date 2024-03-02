
struct BookMyShow {
    n: i32,
    m: i32,
    stree: Vec<[i64; 2]>, // segment tree that tracks (max, sum) of each segment

}

impl BookMyShow {
    fn new(n: i32, m: i32) -> Self {
        let mut stree = vec![[0, 0]; 4 * n as usize];
        BookMyShow::build(&mut stree, 0, 0, n - 1, m);
        BookMyShow { n, m, stree }
    }

    fn build(stree: &mut Vec<[i64; 2]>, i: usize, p: i32, q: i32, m: i32) {
        if p == q {
            stree[i] = [m as i64, m as i64];
            return;
        }
        let r = (p + q) / 2;
        stree[i] = [m as i64, (q - p + 1) as i64 * m as i64];
        BookMyShow::build(stree, 2 * i + 1, p, r, m);
        BookMyShow::build(stree, 2 * i + 2, r + 1, q, m);
    }

    fn query_max(&self, i: usize, p: i32, q: i32, k: i32, max_row: i32) -> Option<Vec<i32>> {
        if p > max_row {
            return None;
        }
        if self.stree[i][0] < k as i64 {
            return None;
        }
        if p == q {
            return Some(vec![p, (self.m - self.stree[i][0] as i32)]);
        }
        let r = (p + q) / 2;
        let ret = self.query_max(2 * i + 1, p, r, k, max_row);
        if let Some(ret) = ret {
            return Some(ret);
        }
        self.query_max(2 * i + 2, r + 1, q, k, max_row)
    }

    fn update_max(&mut self, i: usize, p: i32, q: i32, row: i32, k: i32) {
        if p > row || q < row {
            return;
        }
        if p == q {
            self.stree[i][0] -= k as i64;
            self.stree[i][1] -= k as i64;
            return;
        }
        let r = (p + q) / 2;
        self.stree[i][1] -= k as i64;
        self.update_max(2 * i + 1, p, r, row, k);
        self.update_max(2 * i + 2, r + 1, q, row, k);
        self.stree[i][0] = self.stree[2 * i + 1][0].max(self.stree[2 * i + 2][0]);
    }

    fn query_sum(&self, i: usize, p: i32, q: i32, max_row: i32) -> i64 {
        if p > max_row {
            return 0;
        }
        if q <= max_row {
            return self.stree[i][1];
        }
        let r = (p + q) / 2;
        self.query_sum(2 * i + 1, p, r, max_row) + self.query_sum(2 * i + 2, r + 1, q, max_row)
    }

    fn update_sum(&mut self, i: usize, p: i32, q: i32, k: i32, max_row: i32) {
        if p > max_row {
            return;
        }
        if p == q {
            self.stree[i][0] -= k as i64;
            self.stree[i][1] -= k as i64;
            return;
        }
        let r = (p + q) / 2;
        self.stree[i][1] -= k as i64;
        if r + 1 > max_row || self.stree[2 * i + 1][1] >= k as i64 {
            self.update_sum(2 * i + 1, p, r, k, max_row);
        } else {
            let k = k as i64 - self.stree[2 * i + 1][1];
            self.update_sum(2 * i + 1, p, r, self.stree[2 * i + 1][1] as i32, max_row);
            self.update_sum(2 * i + 2, r + 1, q, k as i32, max_row);
        }
        self.stree[i][0] = self.stree[2 * i + 1][0].max(self.stree[2 * i + 2][0]);
    }

    fn gather(&mut self, k: i32, max_row: i32) -> Vec<i32> {
        if let Some(ret) = self.query_max(0, 0, self.n - 1, k, max_row) {
            self.update_max(0, 0, self.n - 1, ret[0], k);
            ret

        } else {
            vec![]
        }
    }

    fn scatter(&mut self, k: i32, max_row: i32) -> bool {
        let cnt = self.query_sum(0, 0, self.n - 1, max_row);
        if cnt >= k as i64 {
            self.update_sum(0, 0, self.n - 1, k, max_row);
            true

        } else {
            false

        }
    }
}
