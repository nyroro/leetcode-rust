
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let n = card_points.len();
        let k = k as usize;
        let total_sum: i32 = card_points.iter().sum();
        let mut window_sum = 0;
        let mut min_window_sum = 0;

        for i in 0..n - k {
            window_sum += card_points[i];
        }
        min_window_sum = window_sum;

        for i in n - k..n {
            window_sum += card_points[i] - card_points[i - (n - k)];
            min_window_sum = min_window_sum.min(window_sum);
        }

        total_sum - min_window_sum

    }
}
