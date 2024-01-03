
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }
        
        let mut slow = 2;
        for fast in 2..n {
            if nums[fast] != nums[slow - 2] {
                nums[slow] = nums[fast];
                slow += 1;
            }
        }
        
        slow as i32

    }
}
