
use std::collections::HashMap;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        // 统计字符频率

        let mut count: HashMap<char, u32> = HashMap::new();
        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }
        
        // 根据频率排序

        let mut count_vec: Vec<(char, u32)> = count.into_iter().collect();
        count_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        // 构建排序后的字符串

        let mut result = String::new();
        for (c, freq) in count_vec {
            result.push_str(&c.to_string().repeat(freq as usize));
        }
        
        result

    }
}
