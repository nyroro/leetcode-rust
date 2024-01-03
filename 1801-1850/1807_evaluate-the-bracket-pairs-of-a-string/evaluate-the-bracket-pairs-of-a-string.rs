
use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let mut map: HashMap<String, String> = HashMap::new();
        
        // 构建键值对映射

        for pair in knowledge {
            map.insert(pair[0].clone(), pair[1].clone());
        }
        
        let mut result = String::new();
        let mut chars = s.chars().peekable();
        
        while let Some(ch) = chars.next() {
            if ch == '(' {
                let mut key = String::new();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == ')' {
                        chars.next();
                        break;
                    }
                    key.push(chars.next().unwrap());
                }
                
                if let Some(value) = map.get(&key) {
                    result.push_str(value);
                } else {
                    result.push('?');
                }
            } else {
                result.push(ch);
            }
        }
        
        result

    }
}
