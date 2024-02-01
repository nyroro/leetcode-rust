
impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        
        let mut moves = 0;
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                let prev = nums[i];
                nums[i] = nums[i - 1] + 1;
                moves += nums[i] - prev;
            }
        }
        
        moves

    }
}
