
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut combination = Vec::new();
        Self::backtrack(&candidates, target, &mut combination, 0, &mut result);
        result

    }
    
    fn backtrack(candidates: &[i32], target: i32, combination: &mut Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(combination.clone());
            return;
        }
        
        if target < 0 {
            return;
        }
        
        for i in start..candidates.len() {
            combination.push(candidates[i]);
            Self::backtrack(candidates, target - candidates[i], combination, i, result);
            combination.pop();
        }
    }
}
