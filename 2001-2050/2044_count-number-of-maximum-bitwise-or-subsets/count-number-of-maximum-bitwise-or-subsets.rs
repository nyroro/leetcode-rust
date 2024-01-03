
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut max_or = 0;
        let mut count = 0;
        let n = nums.len();

        // 定义回溯函数

        fn backtrack(nums: &Vec<i32>, start: usize, current_or: i32, max_or: &mut i32, count: &mut i32) {
            if start == nums.len() {
                if current_or > *max_or {
                    *max_or = current_or;
                    *count = 1;
                } else if current_or == *max_or {
                    *count += 1;
                }
                return;
            }

            // 不选择当前数字

            backtrack(nums, start + 1, current_or, max_or, count);

            // 选择当前数字

            backtrack(nums, start + 1, current_or | nums[start], max_or, count);
        }

        backtrack(&nums, 0, 0, &mut max_or, &mut count);

        count

    }
}
