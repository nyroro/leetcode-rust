
impl Solution {
    pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let mut i = 0;
        let mut cnt = std::collections::HashMap::new();

        for j in 0..nums.len() {
            *cnt.entry(nums[j]).or_insert(0) += 1;
            let mut max = i32::MIN;
            let mut min = i32::MAX;

            for &k in cnt.keys() {
                max = max.max(k);
                min = min.min(k);
            }

            while max - min > 2 {
                *cnt.get_mut(&nums[i]).unwrap() -= 1;
                if cnt[&nums[i]] == 0 {
                    cnt.remove(&nums[i]);
                }
                i += 1;

                max = i32::MIN;
                min = i32::MAX;
                for &k in cnt.keys() {
                    max = max.max(k);
                    min = min.min(k);
                }
            }

            res += (j - i + 1) as i64;
        }

        res

    }
}
