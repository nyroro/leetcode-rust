
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut missing = n;
        
        for i in 0..n {
            missing ^= i ^ nums[i as usize];
        }
        
        missing

    }
}
