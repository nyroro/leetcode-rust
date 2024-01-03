
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min1 = i32::max_value();
        let mut min2 = i32::max_value();
        
        for num in nums {
            if num <= min1 {
                min1 = num;
            } else if num <= min2 {
                min2 = num;
            } else {
                return true;
            }
        }
        
        false

    }
}
