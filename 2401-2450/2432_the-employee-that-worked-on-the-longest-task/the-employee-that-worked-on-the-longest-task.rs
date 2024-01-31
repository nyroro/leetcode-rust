


impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut ans = logs[0].clone();
        for i in 1..logs.len() {
            let d = logs[i][1] - logs[i - 1][1];
            if d > ans[1] || (d == ans[1] && logs[i][0] < ans[0]) {
                ans = vec![logs[i][0], d];
            }
        }
        ans[0]
    }
}
