
impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut sum = 0;
        for i in 0..time_series.len() {
            if i == time_series.len() - 1 {
                sum += duration;
            } else {
                let diff = time_series[i + 1] - time_series[i];
                sum += diff.min(duration);
            }
        }
        sum

    }
}
