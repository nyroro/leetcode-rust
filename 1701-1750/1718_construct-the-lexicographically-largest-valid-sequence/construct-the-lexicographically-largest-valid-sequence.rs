
impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let mut result = vec![0; 2 * n - 1];
        let mut visited = vec![false; n as usize + 1];
        Self::backtrack(&mut result, &mut visited, n as usize, 0);
        result

    }
    
    fn backtrack(result: &mut Vec<i32>, visited: &mut Vec<bool>, n: usize, index: usize) -> bool {
        if index == result.len() {
            return true;
        }
        
        if result[index] != 0 {
            return Self::backtrack(result, visited, n, index + 1);
        }
        
        for i in (1..=n).rev() {
            if visited[i] {
                continue;
            }
            
            if i == 1 {
                visited[i] = true;
                result[index] = i as i32;
                
                if Self::backtrack(result, visited, n, index + 1) {
                    return true;
                }
                
                visited[i] = false;
                result[index] = 0;
            } else if index + i < result.len() && result[index + i] == 0 {
                visited[i] = true;
                result[index] = i as i32;
                result[index + i] = i as i32;
                
                if Self::backtrack(result, visited, n, index + 1) {
                    return true;
                }
                
                visited[i] = false;
                result[index] = 0;
                result[index + i] = 0;
            }
        }
        
        false

    }
}
