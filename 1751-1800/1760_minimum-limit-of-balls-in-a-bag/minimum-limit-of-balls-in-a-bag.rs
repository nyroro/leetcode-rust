
impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left = 1;
        let mut right = *nums.iter().max().unwrap();

        while left < right {
            let mid = (left + right) / 2;
            let operations = Self::count_operations(&nums, mid, max_operations);

            if operations > max_operations {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left

    }

    fn count_operations(nums: &Vec<i32>, limit: i32, max_operations: i32) -> i32 {
        let mut operations = 0;

        for num in nums {
            operations += (*num - 1) / limit;

            if operations > max_operations {
                break;
            }
        }

        operations

    }
}
