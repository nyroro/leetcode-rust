
impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let (mut low, mut high) = (0, nums[nums.len() - 1] - nums[0]);
        
        while low < high {
            let mid = low + (high - low) / 2;
            let mut count = 0;
            let (mut left, mut right) = (0, 0);
            while right < nums.len() {
                while nums[right] - nums[left] > mid {
                    left += 1;
                }
                count += right - left;
                right += 1;
            }
            if count < k {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low

    }
}
