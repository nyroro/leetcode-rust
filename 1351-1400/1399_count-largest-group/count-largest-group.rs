
use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut digit_sums = HashMap::new();
        
        for num in 1..=n {
            let digit_sum = Solution::calculate_digit_sum(num);
            let count = digit_sums.entry(digit_sum).or_insert(0);
            *count += 1;
        }
        
        let max_count = digit_sums.values().max().unwrap_or(&0);
        let largest_groups = digit_sums.values().filter(|&count| count == max_count).count();
        
        largest_groups as i32

    }
    
    fn calculate_digit_sum(num: i32) -> i32 {
        let mut sum = 0;
        let mut n = num;
        
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        
        sum

    }
}
