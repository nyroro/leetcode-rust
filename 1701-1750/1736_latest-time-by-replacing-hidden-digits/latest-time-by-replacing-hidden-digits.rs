
impl Solution {
    pub fn maximum_time(time: String) -> String {
        let mut chars = time.chars().collect::<Vec<char>>();
        
        if chars[0] == '?' {
            if chars[1] == '?' || chars[1] <= '3' {
                chars[0] = '2';
            } else {
                chars[0] = '1';
            }
        }
        
        if chars[1] == '?' {
            if chars[0] == '2' {
                chars[1] = '3';
            } else {
                chars[1] = '9';
            }
        }
        
        if chars[3] == '?' {
            chars[3] = '5';
        }
        
        if chars[4] == '?' {
            chars[4] = '9';
        }
        
        chars.into_iter().collect()
    }
}
