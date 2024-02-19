
use std::collections::HashSet;
use rand::Rng;

struct Solution {
    n: i32,
    hash_set: HashSet<i32>,
    whitelist: Vec<i32>,
}

impl Solution {
    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let mut hash_set = HashSet::new();
        let mut whitelist = Vec::new();
        let m = blacklist.len() as i32;

        for num in blacklist {
            hash_set.insert(num);
        }

        if m * 2 < n {
            // Approach 1

            Solution {
                n,
                hash_set,
                whitelist,
            }
        } else {
            // Approach 2

            for i in 0..n {
                if !hash_set.contains(&i) {
                    whitelist.push(i);
                }
            }
            Solution {
                n,
                hash_set,
                whitelist,
            }
        }
    }

    fn pick(&self) -> i32 {
        if self.whitelist.is_empty() {
            // Approach 1

            let mut rng = rand::thread_rng();
            loop {
                let val = rng.gen_range(0, self.n);
                if !self.hash_set.contains(&val) {
                    return val;
                }
            }
        } else {
            // Approach 2

            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0, self.whitelist.len());
            self.whitelist[idx]
        }
    }
}
