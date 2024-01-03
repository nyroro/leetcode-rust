
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        let mut used = vec![false; nums.len()];
        Self::backtrack(&nums, &mut used, &mut path, &mut result);
        result

    }
    
    fn backtrack(nums: &Vec<i32>, used: &mut Vec<bool>, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if path.len() == nums.len() {
            result.push(path.clone());
            return;
        }
        
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            
            path.push(nums[i]);
            used[i] = true;
            Self::backtrack(nums, used, path, result);
            path.pop();
            used[i] = false;
        }
    }
}
