
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = String::new();
        Self::generate(&mut result, &mut current, n, n);
        result

    }
    
    fn generate(result: &mut Vec<String>, current: &mut String, left: i32, right: i32) {
        if left == 0 && right == 0 {
            result.push(current.clone());
            return;
        }
        
        if left > 0 {
            current.push('(');
            Self::generate(result, current, left - 1, right);
            current.pop();
        }
        
        if right > left {
            current.push(')');
            Self::generate(result, current, left, right - 1);
            current.pop();
        }
    }
}
