
impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut max_width = 0;

        // 构建递减栈

        for i in 0..nums.len() {
            if stack.is_empty() || nums[i] < nums[*stack.last().unwrap()] {
                stack.push(i);
            }
        }

        // 从右向左遍历数组，计算最大宽度

        for i in (0..nums.len()).rev() {
            while !stack.is_empty() && nums[i] >= nums[*stack.last().unwrap()] {
                max_width = max_width.max(i - stack.pop().unwrap());
            }
        }

        max_width as i32

    }
}
