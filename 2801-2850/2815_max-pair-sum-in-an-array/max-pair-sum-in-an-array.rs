
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = -1;
        
        for i in 0..nums.len() {
            let num1 = nums[i];
            let digit1 = num1.to_string().chars().max().unwrap().to_digit(10).unwrap();
            
            for j in (i + 1)..nums.len() {
                let num2 = nums[j];
                let digit2 = num2.to_string().chars().max().unwrap().to_digit(10).unwrap();
                
                if digit1 == digit2 {
                    max_sum = max_sum.max(num1 + num2);
                }
            }
        }
        
        if max_sum == -1 {
            return -1;
        }
        
        max_sum

    }
}
