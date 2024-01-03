
impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut start = 0;
        let mut end = nums.len() - 1;

        while start < end {
            while start < end && nums[start] % 2 == 0 {
                start += 1;
            }

            while start < end && nums[end] % 2 == 1 {
                end -= 1;
            }

            if start < end {
                nums.swap(start, end);
            }
        }

        nums

    }
}
