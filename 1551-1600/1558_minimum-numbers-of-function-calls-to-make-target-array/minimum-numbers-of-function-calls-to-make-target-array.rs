
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut max_val = 0;
        
        for &num in nums.iter() {
            let mut count = 0;
            let mut n = num;
            
            while n > 0 {
                if n % 2 == 1 {
                    result += 1;
                    n -= 1;
                } else {
                    n /= 2;
                    count += 1;
                }
                max_val = max_val.max(count);
            }
        }
        
        result + max_val

    }
}
