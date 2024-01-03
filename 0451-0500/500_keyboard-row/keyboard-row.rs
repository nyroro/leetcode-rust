
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let rows = vec![
            "qwertyuiop",
            "asdfghjkl",
            "zxcvbnm"
        ];
        
        let mut result = Vec::new();
        
        for word in words {
            let mut row = -1;
            let mut valid = true;
            
            for c in word.chars() {
                let lowercase_c = c.to_lowercase().to_string();
                
                if row == -1 {
                    for (i, r) in rows.iter().enumerate() {
                        if r.contains(&lowercase_c) {
                            row = i as i32;
                            break;
                        }
                    }
                } else {
                    if !rows[row as usize].contains(&lowercase_c) {
                        valid = false;
                        break;
                    }
                }
            }
            
            if valid {
                result.push(word);
            }
        }
        
        result

    }
}
