


impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn solve(i: usize, j: usize, premask: usize, curmask: usize, n: usize, m: usize, dp: &mut Vec<Vec<i64>>) -> i64 {
        if i == n {
            return 1;
        }
        if j == m {
            return Solution::solve(i + 1, 0, curmask, 0, n, m, dp);
        }
        if dp[i][premask] != -1 && j == 0 {
            return dp[i][premask];
        }
        let mut ans = 0;
        for k in 1..=3 {
            if ((premask >> (2 * j)) & 3) != k && (j == 0 || ((curmask >> (2 * j - 2)) & 3) != k) {
                ans += Solution::solve(i, j + 1, premask, curmask | (k << (2 * j)), n, m, dp);
                ans %= Solution::MOD;
            }
        }
        if j == 0 {
            dp[i][premask] = ans;
        }
        ans

    }

    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let n = n as usize;
        let m = m as usize;
        let mut dp = vec![vec![-1; 1 << (2 * m)]; n + 1];
        Solution::solve(0, 0, 0, 0, n, m, &mut dp) as i32

    }
}
