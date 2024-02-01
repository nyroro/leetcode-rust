
use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut arr = BinaryHeap::new();
        let mut s = HashSet::new();
        let mut m = 0;

        arr.push(Reverse(1));
        s.insert(1);

        if n == 1 {
            return 1;
        }

        let mut x = n - 1;
        while x > 0 {
            let t = arr.pop().unwrap().0;
            for &k in &primes {
                let val = t * k;
                if !s.contains(&val) {
                    if arr.len() < n as usize {
                        m = m.max(val);
                    }
                    if arr.len() >= n as usize && val >= m {
                        continue;
                    }
                    arr.push(Reverse(val));
                    s.insert(val);
                }
            }
            x -= 1;
        }
        arr.pop().unwrap().0

    }
}
