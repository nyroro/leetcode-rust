
impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        let p = p as i64;
        let mut total = 0;
        for &num in &nums {
            total += num as i64;
        }
        let target = total % p;
        if target == 0 {
            return 0;
        }
        let mut sum = 0;
        let mut map = std::collections::HashMap::new();
        map.insert(0, -1);
        let mut min_len = n as i32;
        for i in 0..n {
            sum = (sum + nums[i] as i64) % p;
            let diff = (sum + p - target) % p;
            if let Some(&j) = map.get(&diff) {
                min_len = min_len.min((i as i32 - j).max(0));
            }
            map.insert(sum, i as i32);
        }
        if min_len == n as i32 {
            -1

        } else {
            min_len

        }
    }
}
