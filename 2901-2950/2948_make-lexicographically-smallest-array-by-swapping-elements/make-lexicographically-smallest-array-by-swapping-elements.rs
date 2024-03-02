
use std::collections::HashMap; // Import HashMap type


struct DSU {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> DSU {
        let mut par = vec![0; n];
        let mut rank = vec![0; n];
        for i in 0..n {
            par[i] = i;
        }
        DSU { par, rank }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.par[u] == u {
            u

        } else {
            self.par[u] = self.find(self.par[u]);
            self.par[u]
        }
    }

    fn merge(&mut self, u: usize, v: usize) {
        let mut x = self.find(u);
        let mut y = self.find(v);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        } else if self.rank[x] == self.rank[y] {
            self.rank[x] += 1;
        }
        self.par[y] = x;
    }
}

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        let n = nums.len();
        let mut all: Vec<(i32, usize)> = nums.iter().cloned().enumerate().map(|(i, v)| (v, i)).collect(); // Fix type mismatch


        all.sort_unstable_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        let mut dsu = DSU::new(n + 1);
        let mut mp: HashMap<usize, Vec<(i32, usize)>> = HashMap::new(); // Use HashMap type


        mp.insert(all[0].1, vec![(all[0].0, all[0].1)]);
        for i in 1..n {
            if (all[i].0 - all[i - 1].0).abs() <= limit {
                dsu.merge(all[i - 1].1, all[i].1);
                mp.entry(dsu.find(all[i].1))
                    .or_insert(Vec::new())
                    .push((all[i].0, all[i].1));
            } else {
                mp.entry(all[i].1).or_insert(Vec::new()).push((all[i].0, all[i].1));
            }
        }

        let mut ans = vec![0; n];
        for (root, v) in mp {
            let mut x: Vec<usize> = v.iter().map(|&(_, idx)| idx).collect();
            x.sort_unstable();
            for i in 0..x.len() {
                ans[x[i]] = v[i].0;
            }
        }

        ans

    }
}
