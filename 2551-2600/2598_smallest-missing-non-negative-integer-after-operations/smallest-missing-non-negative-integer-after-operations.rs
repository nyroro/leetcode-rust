
use std::collections::HashMap;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        
        for &num in &nums {
            let t = if num >= 0 { num % value } else { (value - num.abs() % value) % value };
            *counter.entry(t).or_insert(0) += 1;
        }
        
        let (mut x, mut y) = (0, i32::MAX);
        for i in 0..value {
            if counter.get(&i).unwrap_or(&0) < &y {
                y = *counter.get(&i).unwrap_or(&0);
                x = i;
            }
        }
        
        y * value + x

    }
}
