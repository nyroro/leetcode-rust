


impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut sl = vec![nums[0]];
        let mut ans = 0;

        for j in 1..n {
            let idx = sl.binary_search(&nums[j]).unwrap_or_else(|x| x);
            sl.insert(idx, nums[j]);

            let mut cnt = if nums[j] < nums[n - 1] { 1 } else { 0 };

            for k in (j + 1..n - 1).rev() {
                if nums[j] < nums[k] {
                    cnt += 1;
                    continue;
                }
                ans += (sl.binary_search(&nums[k]).unwrap_or_else(|x| x) as i64) * cnt;
            }
        }

        ans

    }
}
