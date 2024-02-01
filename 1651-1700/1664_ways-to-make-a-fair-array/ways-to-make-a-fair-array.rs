
impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut even_sum = 0;
        let mut odd_sum = 0;
        let mut count = 0;
        
        for i in 0..nums.len() {
            if i % 2 == 0 {
                even_sum += nums[i];
            } else {
                odd_sum += nums[i];
            }
        }
        
        for i in 0..nums.len() {
            if i % 2 == 0 {
                even_sum -= nums[i];
            } else {
                odd_sum -= nums[i];
            }
            
            if even_sum == odd_sum {
                count += 1;
            }
            
            if i % 2 == 0 {
                even_sum += nums[i];
            } else {
                odd_sum += nums[i];
            }
        }
        
        count

    }
}
