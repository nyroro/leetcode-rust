
impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut prefix_sum = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, -1);
        
        for i in 0..n {
            prefix_sum += nums[i];
            if k != 0 {
                prefix_sum %= k;
            }
            if let Some(&prev) = map.get(&prefix_sum) {
                if i as i32 - prev > 1 {
                    return true;
                }
            } else {
                map.insert(prefix_sum, i as i32);
            }
        }
        
        false

    }
}
