
use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();
        
        let mut t_count: HashMap<char, i32> = HashMap::new();
        let mut window_count: HashMap<char, i32> = HashMap::new();
        
        // 统计字符串 t 中每个字符的出现次数

        for c in t_chars.iter() {
            *t_count.entry(*c).or_insert(0) += 1;
        }
        
        let mut left = 0;
        let mut right = 0;
        let mut min_len = s.len() + 1;
        let mut start = 0;
        let mut count = 0;
        
        while right < s_chars.len() {
            // 扩大窗口

            let c = s_chars[right];
            *window_count.entry(c).or_insert(0) += 1;
            
            // 如果窗口中包含了字符串 t 中的字符，则 count 加一

            if let Some(&t_val) = t_count.get(&c) {
                if let Some(&window_val) = window_count.get(&c) {
                    if window_val <= t_val {
                        count += 1;
                    }
                }
            }
            
            // 缩小窗口

            while count == t_chars.len() {
                if right - left + 1 < min_len {
                    min_len = right - left + 1;
                    start = left;
                }
                
                let c = s_chars[left];
                *window_count.entry(c).or_insert(0) -= 1;
                
                if let Some(&t_val) = t_count.get(&c) {
                    if let Some(&window_val) = window_count.get(&c) {
                        if window_val < t_val {
                            count -= 1;
                        }
                    }
                }
                
                left += 1;
            }
            
            right += 1;
        }
        
        if min_len > s.len() {
            return String::new();
        }
        
        s_chars[start..start+min_len].iter().collect()
    }
}
