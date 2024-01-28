
impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut count = nums[0];
        
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] + 1 {
                count += nums[i];
            } else {
                break;
            }
        }
        
        let mut missing = count;
        while nums.contains(&missing) {
            missing += 1;
        }
        
        missing

    }
}
