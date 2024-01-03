
impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();
        let mut prev_num = 0;
        
        for word in words {
            if let Ok(num) = word.parse::<i32>() {
                if num <= prev_num {
                    return false;
                }
                prev_num = num;
            }
        }
        
        true

    }
}
