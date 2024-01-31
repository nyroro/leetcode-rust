
use std::collections::HashMap;

impl Solution {
    pub fn sum_of_floored_pairs(nums: Vec<i32>) -> i32 {
        let modulo = 1_000_000_007;
        let mut count_map: HashMap<i32, i64> = HashMap::new();
        let mut max_num = 0;

        // Count the occurrences of each number

        for &num in &nums {
            *count_map.entry(num).or_insert(0) += 1;
            max_num = max_num.max(num);
        }

        let mut count = vec![0; (max_num + 1) as usize];

        // Accumulate the counts for each divisor

        for (num, &freq) in &count_map {
            let mut multiple = 1;
            while num * multiple <= max_num {
                count[(num * multiple) as usize] += freq;
                multiple += 1;
            }
        }

        let mut result = 0;
        let mut running_sum = 0;

        // Calculate the sum of floor divisions

        for i in 1..=max_num {
            running_sum += count[i as usize];
            if count_map.contains_key(&i) {
                result += count_map[&i] * running_sum;
            }
        }

        (result % modulo) as i32

    }
}
