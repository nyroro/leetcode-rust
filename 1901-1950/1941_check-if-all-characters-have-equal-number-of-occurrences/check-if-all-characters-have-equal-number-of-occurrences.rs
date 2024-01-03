
use std::collections::HashMap;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut char_count: HashMap<char, i32> = HashMap::new();
        
        // 统计每个字符出现的次数

        for c in s.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }
        
        // 检查所有字符的频率是否相等

        let first_count = char_count.values().next().unwrap();
        char_count.values().all(|count| count == first_count)
    }
}
