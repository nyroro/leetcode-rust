
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut subset = Vec::new();
        Solution::backtrack(&mut result, &mut subset, &nums, 0);
        result

    }
    
    fn backtrack(result: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, nums: &Vec<i32>, start: usize) {
        result.push(subset.clone());
        
        for i in start..nums.len() {
            subset.push(nums[i]);
            Solution::backtrack(result, subset, nums, i + 1);
            subset.pop();
        }
    }
}
