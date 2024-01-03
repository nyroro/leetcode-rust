
impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        use std::collections::HashMap;

        let mut box_counts: HashMap<i32, i32> = HashMap::new();
        let mut max_count = 0;

        for ball in low_limit..=high_limit {
            let sum_digits = Solution::calculate_sum_digits(ball);
            let count = box_counts.entry(sum_digits).or_insert(0);
            *count += 1;
            max_count = max_count.max(*count);
        }

        max_count

    }

    fn calculate_sum_digits(mut num: i32) -> i32 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum

    }
}
