
use std::collections::HashMap;

impl Solution {
    pub fn minimum_seconds(nums: Vec<i32>) -> i32 {
        let mut table: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &t) in nums.iter().enumerate() {
            table.entry(t).or_insert(vec![]).push(i);
        }

        let mut ret = std::i32::MAX;
        for (_, arr) in table.iter() {
            if arr.len() == 1 {
                ret = ret.min(nums.len() as i32 / 2);
            } else {
                let mut v = 0;
                for (&a, &b) in arr.iter().zip(arr.iter().cycle().skip(1)) {
                    v = v.max(((b + nums.len() - a) % nums.len()) as i32 / 2);
                }
                let a = arr[arr.len() - 1];
                let b = nums.len() + arr[0];
                v = v.max(((b - a) % nums.len()) as i32 / 2);
                ret = ret.min(v);
            }
        }
        ret

    }
}
