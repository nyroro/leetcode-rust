
impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut subset = Vec::new();
        Solution::backtrack(0, &nums, &mut subset, &mut result);
        result

    }
    
    fn backtrack(start: usize, nums: &Vec<i32>, subset: &mut Vec<i32>, result: &mut i32) {
        *result += subset.iter().fold(0, |acc, &x| acc ^ x);
        
        for i in start..nums.len() {
            subset.push(nums[i]);
            Solution::backtrack(i + 1, nums, subset, result);
            subset.pop();
        }
    }
}
