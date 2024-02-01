
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut index = nums.len() - 1;
        
        while left <= right {
            let left_square = nums[left].pow(2);
            let right_square = nums[right].pow(2);
            
            if left_square > right_square {
                result[index] = left_square;
                left += 1;
            } else {
                result[index] = right_square;
                right -= 1;
            }
            
            if index == 0 {
                break;
            }
            
            index -= 1;
        }
        
        result

    }
}
