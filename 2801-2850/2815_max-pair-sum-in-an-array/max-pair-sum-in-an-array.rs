
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = -1;
        let mut max_digit = -1;
        
        for i in 0..nums.len() {
            let mut num = nums[i];
            let mut digit = -1;
            
            while num > 0 {
                let curr_digit = num % 10;
                digit = digit.max(curr_digit);
                num /= 10;
            }
            
            for j in 0..i {
                let other_num = nums[j];
                let mut other_digit = -1;
                
                let mut temp = other_num;
                while temp > 0 {
                    let curr_digit = temp % 10;
                    other_digit = other_digit.max(curr_digit);
                    temp /= 10;
                }
                
                if digit == other_digit {
                    max_sum = max_sum.max(num + other_num);
                }
            }
            
            max_digit = max_digit.max(digit);
        }
        
        max_sum

    }
}
