
impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let n = nums.len();
        if n == 1 {
            return nums[0].to_string();
        }
        if n == 2 {
            return format!("{}/{}", nums[0], nums[1]);
        }
        let mut result = nums[0].to_string();
        result.push('/');
        result.push('(');
        for i in 1..n {
            result.push_str(&nums[i].to_string());
            if i != n - 1 {
                result.push('/');
            }
        }
        result.push(')');
        result

    }
}
