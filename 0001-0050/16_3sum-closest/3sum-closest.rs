
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort(); // 排序数组
        
        let mut closest_sum = nums[0] + nums[1] + nums[2]; // 初始化最接近目标值的和
        
        for i in 0..nums.len() - 2 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                
                if sum == target {
                    return sum; // 如果找到和等于目标值的情况，直接返回

                }
                
                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum; // 更新最接近目标值的和

                }
                
                if sum < target {
                    left += 1; // 和小于目标值，左指针右移

                } else {
                    right -= 1; // 和大于目标值，右指针左移

                }
            }
        }
        
        closest_sum

    }
}
