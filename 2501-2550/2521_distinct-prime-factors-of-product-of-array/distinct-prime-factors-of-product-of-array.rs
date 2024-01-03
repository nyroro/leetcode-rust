
use std::collections::HashSet;

impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        let mut product = 1;
        for &num in &nums {
            product *= num;
        }
        
        let mut prime_factors = HashSet::new();
        for &num in &nums {
            let factors = Solution::get_prime_factors(num);
            for factor in factors {
                prime_factors.insert(factor);
            }
        }
        
        prime_factors.len() as i32

    }
    
    fn is_prime(num: i32) -> bool {
        if num <= 1 {
            return false;
        }
        for i in 2..=(num as f64).sqrt() as i32 {
            if num % i == 0 {
                return false;
            }
        }
        true

    }
    
    fn get_prime_factors(num: i32) -> HashSet<i32> {
        let mut factors = HashSet::new();
        let mut n = num;
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                while n % i == 0 {
                    n /= i;
                }
                factors.insert(i);
            }
            i += 1;
        }
        if n > 1 {
            factors.insert(n);
        }
        factors

    }
}
