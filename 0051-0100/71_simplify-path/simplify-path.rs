
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let parts: Vec<&str> = path.split('/').collect();
        let mut stack: Vec<&str> = Vec::new();
        
        for part in parts {
            match part {
                "" | "." => continue,
                ".." => { stack.pop(); },
                _ => stack.push(part),
            }
        }
        
        let simplified_path = stack.join("/");
        
        if simplified_path.is_empty() {
            "/".to_string()
        } else {
            simplified_path

        }
    }
}
