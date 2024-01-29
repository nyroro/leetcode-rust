
use std::collections::HashSet;

impl Solution {
    pub fn min_impossible_or(nums: Vec<i32>) -> i32 {
        let mut bit_set: HashSet<i32> = HashSet::new();

        for &num in nums.iter() {
            if num == num & (-num) {
                bit_set.insert(num);
            }
        }

        let mut i = 1;
        while bit_set.contains(&i) {
            i = i << 1;
        }

        i

    }
}
