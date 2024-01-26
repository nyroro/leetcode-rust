
use std::collections::HashMap;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        let mut t1 = HashMap::new();
        let mut t2 = HashMap::new();

        for i in (0..nums.len()).step_by(2) {
            *t1.entry(nums[i]).or_insert(0) += 1;
        }

        for i in (1..nums.len()).step_by(2) {
            *t2.entry(nums[i]).or_insert(0) += 1;
        }

        let mut a1: Vec<_> = t1.into_iter().collect();
        a1.sort_by_key(|&(_, v)| v);

        let mut a2: Vec<_> = t2.into_iter().collect();
        a2.sort_by_key(|&(_, v)| v);

        if a1.last().unwrap().0 != a2.last().unwrap().0 {
            return nums.len() as i32 - a1.last().unwrap().1 - a2.last().unwrap().1;
        }

        if a1.len() == 1 && a2.len() == 1 {
            return nums.len() as i32 - i32::max(a1.last().unwrap().1, a2.last().unwrap().1);
        } else if a1.len() == 1 {
            return nums.len() as i32 - i32::max(a1.last().unwrap().1 + a2[a2.len() - 2].1, a2.last().unwrap().1);
        } else if a2.len() == 1 {
            return nums.len() as i32 - i32::max(a1.last().unwrap().1, a2.last().unwrap().1 + a1[a1.len() - 2].1);
        } else {
            return nums.len() as i32 - i32::max(a1.last().unwrap().1 + a2[a2.len() - 2].1, a2.last().unwrap().1 + a1[a1.len() - 2].1);
        }
    }
}
