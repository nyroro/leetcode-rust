
impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = Vec::new();
        let mut index = 0;
        Self::backtrack(&nums, &mut result, &mut path, index);
        result

    }
    
    fn backtrack(nums: &Vec<i32>, result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, index: usize) {
        if path.len() >= 2 {
            result.push(path.clone());
        }
        
        let mut used: Vec<bool> = vec![false; 201];
        
        for i in index..nums.len() {
            if !path.is_empty() && nums[i] < *path.last().unwrap() {
                continue;
            }
            
            if used[(nums[i] + 100) as usize] {
                continue;
            }
            
            used[(nums[i] + 100) as usize] = true;
            path.push(nums[i]);
            Self::backtrack(nums, result, path, i + 1);
            path.pop();
        }
    }
}
