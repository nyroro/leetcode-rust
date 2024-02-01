


impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let mut dp = vec![0; n];

        for i in 0..n {
            dp[i] = Solution::dfs(&arr, &mut dp, i as i32, d);
        }

        *dp.iter().max().unwrap()
    }

    fn dfs(arr: &Vec<i32>, dp: &mut Vec<i32>, i: i32, d: i32) -> i32 {
        if dp[i as usize] != 0 {
            return dp[i as usize];
        }

        let mut max_jumps = 1;
        for j in (i - d..i).rev().take_while(|&j| j >= 0 && arr[j as usize] < arr[i as usize]) {
            max_jumps = max_jumps.max(1 + Solution::dfs(arr, dp, j, d));
        }
        for j in (i + 1..=i + d).take_while(|&j| j < arr.len() as i32 && arr[j as usize] < arr[i as usize]) {
            max_jumps = max_jumps.max(1 + Solution::dfs(arr, dp, j, d));
        }

        dp[i as usize] = max_jumps;
        max_jumps

    }
}
