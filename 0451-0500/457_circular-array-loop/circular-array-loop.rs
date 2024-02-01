
impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut nums = nums;

        for i in 0..n {
            let mut slow = i;
            let mut fast = i;

            while nums[slow] * nums[Self::next(&nums, fast)] > 0 && nums[slow] * nums[Self::next(&nums, Self::next(&nums, fast))] > 0 {
                slow = Self::next(&nums, slow);
                fast = Self::next(&nums, Self::next(&nums, fast));

                if slow == fast {
                    if slow == Self::next(&nums, slow) {
                        return false;
                    }
                    return true;
                }
            }

            let mut j = i;
            let val = nums[i];
            while nums[j] * val > 0 {
                let nexx = Self::next(&nums, j);
                nums[j] = 0;
                j = nexx;
            }
        }

        false

    }

    fn next(nums: &Vec<i32>, i: usize) -> usize {
        let n = nums.len();
        ((n as i32 + nums[i] + i as i32) % n as i32) as usize

    }
}
