
use std::collections::HashSet;

impl Solution {
    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        let mut distinct_nums = HashSet::new();
        
        for num in nums {
            let rev_num = num.to_string().chars().rev().collect::<String>().parse::<i32>().unwrap();
            distinct_nums.insert(rev_num);
        }
        
        distinct_nums.len() as i32

    }
}
