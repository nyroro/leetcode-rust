
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort(); // 对候选数组进行排序

        let mut result = Vec::new();
        let mut combination = Vec::new();
        Self::backtrack(&candidates, target, 0, &mut combination, &mut result);
        result

    }
    
    fn backtrack(candidates: &[i32], target: i32, start: usize, combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(combination.clone()); // 找到一个满足条件的组合

            return;
        }
        
        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue; // 避免重复的组合

            }
            
            if candidates[i] > target {
                break; // 剪枝操作，提前结束搜索

            }
            
            combination.push(candidates[i]);
            Self::backtrack(candidates, target - candidates[i], i + 1, combination, result); // 递归搜索下一个位置的候选元素

            combination.pop();
        }
    }
}
