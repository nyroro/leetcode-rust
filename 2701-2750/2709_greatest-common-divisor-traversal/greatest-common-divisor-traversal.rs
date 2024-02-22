
struct UnionFind {
    par: Vec<usize>,
    sz: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> UnionFind {
        UnionFind {
            par: (0..size).collect(),
            sz: vec![1; size],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x

        } else {
            let root = self.find(self.par[x]);
            self.par[x] = root;
            root

        }
    }

    fn unite(&mut self, a: usize, b: usize) {
        let mut apar = self.find(a);
        let mut bpar = self.find(b);

        if apar == bpar {
            return;
        }

        if self.sz[apar] < self.sz[bpar] {
            std::mem::swap(&mut apar, &mut bpar);
        }

        self.sz[apar] += self.sz[bpar];
        self.sz[bpar] = 0;
        self.par[bpar] = self.par[apar];
    }
}



impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        let mut uf = UnionFind::new(100100);
        let mut mp = std::collections::HashMap::new();

        if nums.len() == 1 {
            return true;
        }

        for &x in &nums {
            *mp.entry(x).or_insert(0) += 1;
            if x == 1 {
                return false;
            }
        }

        let mut sieve = vec![0; 100100];
        for i in 1..100100 {
            sieve[i] = i;
            if let Some(&sz) = mp.get(&(i as i32)) {
                uf.sz[i] = sz as usize;
            } else {
                uf.sz[i] = 0;
            }
            uf.par[i] = i;
        }

        for i in 2..100100 {
            if sieve[i] == i {
                let mut j = i + i;
                while j < 100100 {
                    sieve[j] = i;
                    j += i;
                }
            }
        }

        for &x in &nums {
            let mut mp2 = std::collections::HashMap::new();
            let mut cpy = x;
            while cpy > 1 {
                *mp2.entry(sieve[cpy as usize]).or_insert(0) += 1;
                cpy = cpy / sieve[cpy as usize] as i32;
            }

            for (val, _) in mp2 {
                uf.unite(x as usize, val);
            }
        }

        let mut cmp = 0;
        for i in 2..100100 {
            if uf.par[i] == i && uf.sz[i] > 0 {
                cmp += 1;
            }
        }

        cmp == 1

    }
}
