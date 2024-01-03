
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut increasing = true;
        let mut decreasing = true;
        
        for i in 1..n {
            if nums[i] < nums[i - 1] {
                increasing = false;
            }
            if nums[i] > nums[i - 1] {
                decreasing = false;
            }
        }
        
        increasing || decreasing

    }
}
