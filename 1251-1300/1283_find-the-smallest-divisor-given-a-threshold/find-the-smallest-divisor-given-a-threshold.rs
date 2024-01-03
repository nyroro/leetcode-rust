
impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut left = 1;
        let mut right = *nums.iter().max().unwrap();

        while left <= right {
            let divisor = (left + right) / 2;
            let sum = Solution::calculate_sum(&nums, divisor);

            if sum <= threshold {
                right = divisor - 1;
            } else {
                left = divisor + 1;
            }
        }

        left

    }

    fn calculate_sum(nums: &Vec<i32>, divisor: i32) -> i32 {
        let mut sum = 0;

        for num in nums {
            sum += (num + divisor - 1) / divisor;
        }

        sum

    }
}
