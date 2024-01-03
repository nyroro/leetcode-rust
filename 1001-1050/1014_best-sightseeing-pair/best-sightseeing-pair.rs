
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let mut max_score = 0;
        let mut prev_max = values[0] + 0;

        for i in 1..values.len() {
            max_score = max_score.max(prev_max + values[i] - i as i32);
            prev_max = prev_max.max(values[i] + i as i32);
        }

        max_score

    }
}
