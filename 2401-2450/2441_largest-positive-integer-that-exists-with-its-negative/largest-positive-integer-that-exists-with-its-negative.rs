
impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let mut max_k = -1;

        for &num in nums.iter() {
            if set.contains(&-num) {
                max_k = max_k.max(num);
            }
            set.insert(num);
        }

        max_k

    }
}
