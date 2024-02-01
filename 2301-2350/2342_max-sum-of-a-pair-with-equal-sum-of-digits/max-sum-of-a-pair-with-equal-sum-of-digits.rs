
use std::collections::HashMap;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = -1;
        let mut digit_sum_map: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            let mut sum = 0;
            let mut n = num;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }

            if let Some(&prev_num) = digit_sum_map.get(&sum) {
                max_sum = max_sum.max(prev_num + num);
            }

            digit_sum_map.entry(sum).or_insert(num);
        }

        max_sum

    }
}
