
use std::collections::HashMap;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        let mut res: i64 = 0;
        let mut cnt: i32 = 0;
        let mut l: usize = 0;
        let mut hm: HashMap<i32, i32> = HashMap::new();

        for (r, &v) in nums.iter().enumerate() {
            *hm.entry(v).or_insert(0) += 1;
            cnt += hm[&v] - 1;

            while cnt >= k {
                hm.entry(nums[l]).and_modify(|e| *e -= 1);
                cnt -= hm[&nums[l]];
                l += 1;
            }

            res += (r - l + 1) as i64;
        }

        let n = nums.len() as i64;
        return (n + 1) * n / 2 - res;
    }
}
