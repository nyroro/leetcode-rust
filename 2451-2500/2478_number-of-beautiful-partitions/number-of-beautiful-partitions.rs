


impl Solution {
    pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
        let mod_val = 1_000_000_007;
        let n = s.len();
        let mut dp = vec![vec![-1; (k + 1) as usize]; (n + 1) as usize];
        
        fn solve(s: &Vec<char>, k: i32, min_length: i32, dp: &mut Vec<Vec<i32>>, i: usize) -> i32 {
            let n = s.len();
            let mod_val = 1_000_000_007;
            if k < 0 || (n - i) < (min_length * k) as usize {
                return 0;
            }
            if i >= n {
                if k == 0 {
                    return 1;
                }
                return 0;
            }
            if dp[i][k as usize] != -1 {
                return dp[i][k as usize];
            }
            let mut a = 0;
            for j in i + min_length as usize - 1..n.min(n - (k - 1) as usize * min_length as usize) {
                if (s[i] == '2' || s[i] == '3' || s[i] == '5' || s[i] == '7') && (s[j] == '1' || s[j] == '4' || s[j] == '6' || s[j] == '8' || s[j] == '9') {
                    a = (solve(s, k - 1, min_length, dp, j + 1) + a) % mod_val;
                }
            }
            dp[i][k as usize] = a % mod_val;
            a % mod_val

        }
        
        let s: Vec<char> = s.chars().collect();
        solve(&s, k, min_length, &mut dp, 0)
    }
}
