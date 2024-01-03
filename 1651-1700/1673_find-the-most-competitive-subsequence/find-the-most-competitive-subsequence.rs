
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut stack = Vec::new();
        let n = nums.len();
        for (i, &num) in nums.iter().enumerate() {
            while !stack.is_empty() && stack.len() + n - i > k as usize && *stack.last().unwrap() > num {
                stack.pop();
            }
            if stack.len() < k as usize {
                stack.push(num);
            }
        }
        stack

    }
}
