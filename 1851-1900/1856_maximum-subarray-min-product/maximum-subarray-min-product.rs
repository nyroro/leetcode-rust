
impl Solution {
    pub fn max_sum_min_product(nums: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let mut left: Vec<i32> = vec![0; nums.len()];
        let mut right: Vec<i32> = vec![0; nums.len()];

        for i in 0..nums.len() {
            while let Some(&top) = stack.last() {
                if nums[top] < nums[i] {
                    break;
                }
                stack.pop();
            }
            left[i] = if stack.is_empty() { -1 } else { stack.last().unwrap().clone() as i32 };
            stack.push(i);
        }

        stack.clear();

        for i in (0..nums.len()).rev() {
            while let Some(&top) = stack.last() {
                if nums[top] < nums[i] {
                    break;
                }
                stack.pop();
            }
            right[i] = if stack.is_empty() { nums.len() as i32 } else { stack.last().unwrap().clone() as i32 };
            stack.push(i);
        }

        let mut prefix_sum: Vec<i64> = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i] as i64;
        }

        let mut max_product: i64 = 0;
        for i in 0..nums.len() {
            let sum = prefix_sum[right[i] as usize] - prefix_sum[left[i] as usize + 1];
            max_product = max_product.max(nums[i] as i64 * sum);
        }

        (max_product % 1_000_000_007) as i32

    }
}
