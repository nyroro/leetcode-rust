
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x; // 计算目标值，即剩余的x值

        let n = nums.len();
        let mut left = 0;
        let mut right = 0;
        let mut sum = 0;
        let mut min_ops = n as i32 + 1; // 初始化最小操作次数为数组长度加1


        while right < n {
            sum += nums[right]; // 右指针向右移动，累加元素值

            while sum > target && left <= right {
                sum -= nums[left]; // 左指针向右移动，减去元素值

                left += 1;
            }
            if sum == target {
                min_ops = min_ops.min((n - (right - left + 1)) as i32); // 更新最小操作次数

            }
            right += 1;
        }

        if min_ops > n as i32 {
            -1 // 如果最小操作次数大于数组长度，则返回-1

        } else {
            min_ops

        }
    }
}
