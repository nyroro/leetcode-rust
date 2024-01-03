
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut temp = Vec::new();
        Self::backtrack(k, n, 1, 0, &mut temp, &mut result);
        result

    }
    
    fn backtrack(k: i32, n: i32, start: i32, sum: i32, temp: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if temp.len() == k as usize && sum == n {
            result.push(temp.clone());
            return;
        }
        
        if temp.len() < k as usize && sum < n {
            for i in start..=9 {
                temp.push(i);
                Self::backtrack(k, n, i + 1, sum + i, temp, result);
                temp.pop();
            }
        }
    }
}
