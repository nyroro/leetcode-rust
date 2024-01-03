
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let n = strs.len();
        let m = strs[0].len();
        let mut dp = vec![1; m];
        for i in 1..m {
            for j in 0..i {
                let mut valid = true;
                for k in 0..n {
                    if strs[k].as_bytes()[j] > strs[k].as_bytes()[i] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        (m - dp.iter().max().unwrap()) as i32

    }
}
