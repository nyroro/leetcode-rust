
impl Solution {
    pub fn maximize_greatness(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        let mut i = 0;
        for (index, &num) in nums.iter().enumerate() {
            if nums[i] < num {
                i += 1;
            }
        }

        i as i32

    }
}
