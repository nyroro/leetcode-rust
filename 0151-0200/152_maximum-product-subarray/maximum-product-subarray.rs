
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_product = nums[0];
        let mut min_product = nums[0];
        let mut result = nums[0];
        
        for i in 1..nums.len() {
            if nums[i] < 0 {
                std::mem::swap(&mut max_product, &mut min_product);
            }
            
            max_product = std::cmp::max(nums[i], max_product * nums[i]);
            min_product = std::cmp::min(nums[i], min_product * nums[i]);
            
            result = std::cmp::max(result, max_product);
        }
        
        result

    }
}
