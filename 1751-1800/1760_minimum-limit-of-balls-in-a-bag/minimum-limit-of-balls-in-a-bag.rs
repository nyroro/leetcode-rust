
impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left = 1;
        let mut right = *nums.iter().max().unwrap() * max_operations;

        while left < right {
            let mid = (left + right) / 2;
            let max_balls = Self::count_operations(&nums, mid, max_operations);

            if max_balls > mid {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left

    }

    fn count_operations(nums: &Vec<i32>, limit: i32, max_operations: i32) -> i32 {
        let mut count = 0;

        for num in nums {
            count += (*num - 1) / limit;

            if count > max_operations {
                break;
            }
        }

        count

    }
}
