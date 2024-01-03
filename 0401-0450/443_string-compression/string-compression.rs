
impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut i = 0;
        let mut count = 1;
        
        for j in 1..chars.len() {
            if chars[j] == chars[j - 1] {
                count += 1;
            } else {
                chars[i] = chars[j - 1];
                i += 1;
                
                if count > 1 {
                    let count_chars: Vec<char> = count.to_string().chars().collect();
                    for c in count_chars {
                        chars[i] = c;
                        i += 1;
                    }
                }
                
                count = 1;
            }
        }
        
        chars[i] = chars[chars.len() - 1];
        i += 1;
        
        if count > 1 {
            let count_chars: Vec<char> = count.to_string().chars().collect();
            for c in count_chars {
                chars[i] = c;
                i += 1;
            }
        }
        
        chars.resize(i, ' ');
        
        i as i32

    }
}
