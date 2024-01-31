


impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();
        let mut dp = vec![vec![-1; 2]; n];

        fn solve(nums: &Vec<i32>, x: i32, i: usize, p: i32, dp: &mut Vec<Vec<i64>>) -> i64 {
            if i >= nums.len() {
                return 0;
            }

            if dp[i][p as usize] != -1 {
                return dp[i][p as usize];
            }

            let y = nums[i] % 2;
            let include = if p == y {
                nums[i] as i64 + solve(nums, x, i + 1, y, dp)
            } else {
                nums[i] as i64 - x as i64 + solve(nums, x, i + 1, y, dp)
            };

            let exclude = solve(nums, x, i + 1, p, dp);

            dp[i][p as usize] = include.max(exclude);
            dp[i][p as usize]
        }

        let p = nums[0] % 2;
        (nums[0] as i64 + solve(&nums, x, 1, p, &mut dp)).max(0)
    }
}
