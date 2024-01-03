
impl Solution {
    pub fn remove_comments(source: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut in_block_comment = false;
        let mut new_line = String::new();
        
        for line in source {
            let mut chars = line.chars().peekable();
            while let Some(ch) = chars.next() {
                if in_block_comment {
                    if ch == '*' && chars.peek() == Some(&'/') {
                        in_block_comment = false;
                        chars.next();
                    }
                } else {
                    if ch == '/' && chars.peek() == Some(&'/') {
                        break;
                    } else if ch == '/' && chars.peek() == Some(&'*') {
                        in_block_comment = true;
                        chars.next();
                    } else {
                        new_line.push(ch);
                    }
                }
            }
            
            if !in_block_comment && !new_line.is_empty() {
                result.push(new_line);
                new_line = String::new();
            }
        }
        
        result

    }
}
