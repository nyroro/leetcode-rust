
impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left_sum = 0;
        let total_sum: i32 = nums.iter().sum();
        let mut scores = vec![total_sum];

        for (i, &num) in nums.iter().enumerate() {
            left_sum += num;
            scores.push((i as i32 + 1 - left_sum + total_sum - left_sum));
        }

        let max_score = *scores.iter().max().unwrap();
        let mut result = Vec::new();

        for (i, &score) in scores.iter().enumerate() {
            if score == max_score {
                result.push(i as i32);
            }
        }

        result

    }
}
