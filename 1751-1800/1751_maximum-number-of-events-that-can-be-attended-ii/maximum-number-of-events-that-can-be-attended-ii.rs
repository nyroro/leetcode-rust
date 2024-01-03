
impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events;
        events.sort_by(|a, b| a[1].cmp(&b[1]));
        let n = events.len();
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; n + 1];
        for i in 1..=n {
            for j in 1..=k {
                let mut l = 0;
                let mut r = i - 1;
                while l < r {
                    let m = l + (r - l) / 2;
                    if events[m][1] < events[i - 1][0] {
                        l = m + 1;
                    } else {
                        r = m;
                    }
                }
                let idx = if events[l][1] < events[i - 1][0] { l + 1 } else { l };
                dp[i][j] = dp[i - 1][j].max(dp[idx][j - 1] + events[i - 1][2]);
            }
        }
        dp[n][k]
    }
}
