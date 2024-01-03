
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        
        // 计算每个长度为 k 的子数组的和

        let mut sums = vec![0; n - k + 1];
        let mut sum = 0;
        for i in 0..k {
            sum += nums[i];
        }
        sums[0] = sum;
        for i in k..n {
            sum += nums[i] - nums[i - k];
            sums[i - k + 1] = sum;
        }
        
        // 计算左侧最大和子数组的起始位置

        let mut left = vec![0; n - k + 1];
        let mut max_sum = 0;
        for i in 0..n - k + 1 {
            if sums[i] > max_sum {
                max_sum = sums[i];
                left[i] = i;
            } else {
                left[i] = left[i - 1];
            }
        }
        
        // 计算右侧最大和子数组的起始位置

        let mut right = vec![0; n - k + 1];
        max_sum = 0;
        for i in (k..n - k + 1).rev() {
            if sums[i] >= max_sum {
                max_sum = sums[i];
                right[i] = i;
            } else {
                right[i] = right[i + 1];
            }
        }
        
        // 找到三个非重叠子数组的最大和及其起始位置

        let mut max_sum = 0;
        let mut result = vec![0; 3];
        for i in k..n - 2 * k + 1 {
            let sum = sums[i] + sums[left[i - k]] + sums[right[i + k]];
            if sum > max_sum {
                max_sum = sum;
                result = vec![left[i - k] as i32, i as i32, right[i + k] as i32];
            }
        }
        
        result

    }
}
