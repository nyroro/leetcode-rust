
impl Solution {
    fn compute_kmp(s: &str) -> Vec<usize> {
        let n = s.len();
        let mut arr = vec![0; n];
        let mut i = 1;
        let mut len = 0;
        while i < n {
            if s.chars().nth(len) == s.chars().nth(i) {
                len += 1;
                arr[i] = len;
                i += 1;
            } else {
                if len == 0 {
                    i += 1;
                } else {
                    len = arr[len - 1];
                }
            }
        }
        arr

    }

    fn fun(dp: &mut Vec<Vec<Vec<Vec<Option<i32>>>>>, i: usize, j: usize, flag1: bool, flag2: bool, s1: &str, s2: &str, evil: &str, kmp: &Vec<usize>, modulo: i32) -> i32 {
        if j == evil.len() {
            return 0;
        }
        if i == s1.len() {
            return 1;
        }
        if let Some(val) = dp[i][j][flag1 as usize][flag2 as usize] {
            return val;
        }
        let from = if flag1 { s1.chars().nth(i).unwrap() } else { 'a' };
        let to = if flag2 { s2.chars().nth(i).unwrap() } else { 'z' };
        let mut ans = 0;
        for ch in from..=to {
            if ch == evil.chars().nth(j).unwrap() {
                ans += Solution::fun(dp, i + 1, j + 1, flag1 && (ch == from), flag2 && (ch == to), s1, s2, evil, kmp, modulo);
                ans %= modulo;
            } else {
                let mut x = j;
                while x > 0 && evil.chars().nth(x).unwrap() != ch {
                    x = kmp[x - 1];
                }
                if evil.chars().nth(x).unwrap() == ch {
                    x += 1;
                }
                ans += Solution::fun(dp, i + 1, x, flag1 && (ch == from), flag2 && (ch == to), s1, s2, evil, kmp, modulo);
                ans %= modulo;
            }
        }
        dp[i][j][flag1 as usize][flag2 as usize] = Some(ans);
        ans

    }

    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![vec![vec![None; 2]; 2]; evil.len() + 1]; n + 1];
        let modulo = 1_000_000_007;
        let kmp = Solution::compute_kmp(&evil);
        Solution::fun(&mut dp, 0, 0, true, true, &s1, &s2, &evil, &kmp, modulo)
    }
}
