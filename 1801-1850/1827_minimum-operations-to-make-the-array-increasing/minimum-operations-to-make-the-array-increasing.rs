
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut operations = 0;
        
        for i in 1..nums.len() {
            if nums[i] <= nums[i-1] {
                let diff = nums[i-1] - nums[i] + 1;
                nums[i] += diff;
                operations += diff;
            }
        }
        
        operations

    }
}
