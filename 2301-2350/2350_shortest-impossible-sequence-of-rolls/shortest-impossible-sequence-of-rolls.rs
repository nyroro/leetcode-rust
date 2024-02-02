
use std::collections::HashSet;

impl Solution {
    pub fn shortest_sequence(rolls: Vec<i32>, k: i32) -> i32 {
        let mut set = HashSet::new();
        let mut res = 0;

        for r in rolls {
            set.insert(r);
            if set.len() == k as usize {
                res += 1;
                set.clear();
            }
        }

        res + 1

    }
}
