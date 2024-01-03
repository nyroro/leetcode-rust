
use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // 如果两个字符串长度不相等，直接返回false

        if s.len() != t.len() {
            return false;
        }
        
        let mut counter: HashMap<char, i32> = HashMap::new();
        
        // 统计字符串s中每个字母的出现次数

        for c in s.chars() {
            *counter.entry(c).or_insert(0) += 1;
        }
        
        // 遍历字符串t，对于每个字母，将其在哈希表中的计数减1

        for c in t.chars() {
            if let Some(count) = counter.get_mut(&c) {
                *count -= 1;
                // 如果计数小于0，说明t中出现了s中没有的字母，直接返回false

                if *count < 0 {
                    return false;
                }
            } else {
                // 如果t中出现了s中没有的字母，直接返回false

                return false;
            }
        }
        
        // 检查哈希表中所有字母的计数是否为0

        for count in counter.values() {
            if *count != 0 {
                return false;
            }
        }
        
        // 返回true

        true

    }
}
