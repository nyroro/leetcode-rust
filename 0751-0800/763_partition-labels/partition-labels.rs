
use std::collections::HashMap; // 导入HashMap


impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_occurrence = HashMap::new();
        let mut result = Vec::new();
        
        // 记录每个字母最后出现的位置

        for (i, c) in s.chars().enumerate() {
            last_occurrence.insert(c, i);
        }
        
        let mut start = 0;
        let mut end = 0;
        
        // 遍历字符串，确定每个部分的起始位置和结束位置

        for (i, c) in s.chars().enumerate() {
            if let Some(&last) = last_occurrence.get(&c) {
                if last > end {
                    end = last;
                }
                
                if i == end {
                    result.push((end - start + 1) as i32);
                    start = i + 1;
                }
            }
        }
        
        result

    }
}
