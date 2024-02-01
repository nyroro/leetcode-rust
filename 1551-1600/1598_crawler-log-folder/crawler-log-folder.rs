
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut stack: Vec<&str> = Vec::new();
        
        for log in logs {
            match log.as_str() {
                "../" => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                },
                "./" => {},
                _ => {
                    stack.push(log.trim_end_matches('/'));
                }
            }
        }
        
        stack.len() as i32

    }
}
