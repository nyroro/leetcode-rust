
impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(0, 0);
        let mut sum = 0;
        let mut count = 0;
        let mut last_end = 0;
        for (i, &num) in nums.iter().enumerate() {
            sum += num;
            if let Some(&j) = map.get(&(sum - target)) {
                if j >= last_end {
                    count += 1;
                    last_end = i + 1;
                }
            }
            map.insert(sum, i + 1);
        }
        count

    }
}
