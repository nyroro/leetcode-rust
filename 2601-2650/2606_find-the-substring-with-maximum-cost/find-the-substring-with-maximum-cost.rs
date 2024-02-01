
impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut char_vals = HashMap::new();
        for (i, c) in chars.chars().enumerate() {
            char_vals.insert(c, vals[i]);
        }
        
        let mut left = 0;
        let mut right = 0;
        let mut max_cost = 0;
        let mut current_cost = 0;
        
        let s_chars: Vec<char> = s.chars().collect();
        
        while right < s_chars.len() {
            let c = s_chars[right];
            if let Some(&val) = char_vals.get(&c) {
                current_cost += val;
            } else {
                current_cost += (c as u8 - b'a' + 1) as i32;
            }
            
            while left <= right && current_cost < 0 {
                let c = s_chars[left];
                if let Some(&val) = char_vals.get(&c) {
                    current_cost -= val;
                } else {
                    current_cost -= (c as u8 - b'a' + 1) as i32;
                }
                left += 1;
            }
            
            max_cost = max_cost.max(current_cost);
            right += 1;
        }
        
        max_cost

    }
}
