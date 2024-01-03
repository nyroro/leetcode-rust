
use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut num = n;
        let mut set = HashSet::new();
        
        while num != 1 && !set.contains(&num) {
            set.insert(num);
            let mut sum = 0;
            while num > 0 {
                let digit = num % 10;
                sum += digit * digit;
                num /= 10;
            }
            num = sum;
        }
        
        num == 1

    }
}
