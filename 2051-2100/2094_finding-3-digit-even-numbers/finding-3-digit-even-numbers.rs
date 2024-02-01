
impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut path = Vec::new();
        let mut visited = vec![false; digits.len()];
        
        Self::backtrack(&digits, &mut visited, &mut path, &mut result);
        
        result.sort();
        result

    }
    
    fn backtrack(digits: &Vec<i32>, visited: &mut Vec<bool>, path: &mut Vec<i32>, result: &mut Vec<i32>) {
        if path.len() == 3 {
            let num = path.iter().fold(0, |acc, &x| acc * 10 + x);
            if num % 2 == 0 {
                result.push(num);
            }
            return;
        }
        
        for i in 0..digits.len() {
            if visited[i] {
                continue;
            }
            if i > 0 && digits[i] == digits[i - 1] && !visited[i - 1] {
                continue;
            }
            visited[i] = true;
            path.push(digits[i]);
            Self::backtrack(digits, visited, path, result);
            visited[i] = false;
            path.pop();
        }
    }
}
