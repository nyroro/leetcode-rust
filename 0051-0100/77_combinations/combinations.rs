
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut combination = Vec::new();
        Self::backtrack(1, n, k, &mut combination, &mut result);
        result

    }
    
    fn backtrack(start: i32, n: i32, k: i32, combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if k == 0 {
            result.push(combination.clone());
            return;
        }
        
        for i in start..=n {
            combination.push(i);
            Self::backtrack(i + 1, n, k - 1, combination, result);
            combination.pop();
        }
    }
}
