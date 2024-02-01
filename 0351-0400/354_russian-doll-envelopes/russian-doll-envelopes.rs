
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_by(|a, b| {
            if a[0] == b[0] {
                b[1].cmp(&a[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        
        let mut dp = vec![1; envelopes.len()];
        let mut max_count = 1;
        
        for i in 1..envelopes.len() {
            for j in 0..i {
                if envelopes[i][1] > envelopes[j][1] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            max_count = max_count.max(dp[i]);
        }
        
        max_count

    }
}
