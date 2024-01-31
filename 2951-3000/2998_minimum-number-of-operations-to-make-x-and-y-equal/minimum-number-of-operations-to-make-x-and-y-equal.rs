
use std::collections::{HashSet, VecDeque};



impl Solution {
    pub fn minimum_operations_to_make_equal(x: i32, y: i32) -> i32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(x);
        let mut ans = 0;

        while let Some(sz) = queue.len().checked_sub(1) {
            for _ in 0..=sz {
                if let Some(k) = queue.pop_front() {
                    if k == y {
                        return ans;
                    }

                    if k % 11 == 0 && !visited.contains(&(k / 11)) {
                        visited.insert(k / 11);
                        queue.push_back(k / 11);
                    }

                    if k % 5 == 0 && !visited.contains(&(k / 5)) {
                        visited.insert(k / 5);
                        queue.push_back(k / 5);
                    }

                    if k > 0 && !visited.contains(&(k - 1)) {
                        visited.insert(k - 1);
                        queue.push_back(k - 1);
                    }

                    if !visited.contains(&(k + 1)) {
                        visited.insert(k + 1);
                        queue.push_back(k + 1);
                    }
                }
            }
            ans += 1;
        }

        ans

    }
}
