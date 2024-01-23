
use std::collections::HashMap;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut char_count = HashMap::new();
        for c in s.chars() {
            let count = char_count.entry(c).or_insert(0);
            *count += 1;
        }
        
        let mut s_chars: Vec<char> = s.chars().collect();
        s_chars.sort_by(|a, b| b.cmp(a));
        
        let mut now = 0;
        let mut pre = ' ';
        let mut j = 0;
        for i in 0..s_chars.len() {
            if pre == s_chars[i] {
                now += 1;
            } else {
                pre = s_chars[i];
                now = 1;
            }
            if now > repeat_limit {
                j = std::cmp::max(i + 1, j);
                while j < s_chars.len() && pre == s_chars[j] {
                    j += 1;
                }
                if j < s_chars.len() {
                    s_chars.swap(i, j);
                    now = 0;
                    pre = ' ';
                } else {
                    s_chars.truncate(i);
                    break;
                }
            }
        }
        
        s_chars.iter().collect()
    }
}
