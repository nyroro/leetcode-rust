

impl Solution {
    pub fn max_increasing_groups(usage_limits: Vec<i32>) -> i32 {
        let mut usage_limits = usage_limits;
        usage_limits.sort(); // Sort the input vector in ascending order

        let n = usage_limits.len();
        let mut ans = 0_i64;
        let mut extra = 0_i64;
        let mut i = 0;

        while i < n {
            extra += usage_limits[i] as i64;
            if extra > ans {
                ans += 1;
                extra -= ans;
            }
            i += 1;
        }

        ans as i32// Return the maximum number of groups

    }
}
