
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        // 检查字符串长度是否相等

        if s.len() != goal.len() {
            return false;
        }
        
        // 检查字符串是否相等

        if s == goal {
            // 检查是否存在重复字符

            let mut char_set = std::collections::HashSet::new();
            for ch in s.chars() {
                if !char_set.insert(ch) {
                    return true;
                }
            }
            return false;
        }
        
        // 找到两个不同字符的位置

        let mut first_diff = -1;
        let mut second_diff = -1;
        for (i, (ch1, ch2)) in s.chars().zip(goal.chars()).enumerate() {
            if ch1 != ch2 {
                if first_diff == -1 {
                    first_diff = i as i32;
                } else if second_diff == -1 {
                    second_diff = i as i32;
                } else {
                    return false;
                }
            }
        }
        
        // 检查交换后的字符串是否相等

        if first_diff != -1 && second_diff != -1 {
            let mut s_chars: Vec<char> = s.chars().collect();
            s_chars.swap(first_diff as usize, second_diff as usize);
            let swapped_s: String = s_chars.into_iter().collect();
            return swapped_s == goal;
        }
        
        false

    }
}
