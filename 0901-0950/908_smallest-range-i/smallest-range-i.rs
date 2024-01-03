
impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let min_val = nums.iter().min().unwrap();
        let max_val = nums.iter().max().unwrap();
        let diff = max_val - min_val;
        
        if diff <= 2 * k {
            0

        } else {
            diff - 2 * k

        }
    }
}
