


impl Solution {
    pub fn box_delivering(boxes: Vec<Vec<i32>>, ports_count: i32, mut max_boxes: i32, mut max_weight: i32) -> i32 {
        let n = boxes.len();
        let mut need = 0;
        let mut j = 0;
        let mut last_j = 0;
        let mut dp = vec![200000; n + 1];
        dp[0] = 0;

        for i in 0..n {
            while j < n && max_boxes > 0 && max_weight >= boxes[j][1] {
                max_boxes -= 1;
                max_weight -= boxes[j][1];
                if j == 0 || boxes[j][0] != boxes[j - 1][0] {
                    last_j = j;
                    need += 1;
                }
                j += 1;
            }
            dp[j] = dp[j].min(dp[i] + need + 1);
            dp[last_j] = dp[last_j].min(dp[i] + need);
            max_boxes += 1;
            max_weight += boxes[i][1];
            if i == n - 1 || boxes[i][0] != boxes[i + 1][0] {
                need -= 1;
            }
        }
        dp[n]
    }
}
