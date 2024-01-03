
use std::collections::HashMap;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut count_map: HashMap<&str, i32> = HashMap::new();
        
        // 统计每个字符串的出现次数

        for s in &arr {
            *count_map.entry(s).or_insert(0) += 1;
        }
        
        let mut distinct_count = 0;
        
        // 遍历数组，找到第k个出现次数为1的字符串

        for s in &arr {
            if count_map.get(s.as_str()) == Some(&1) {
                distinct_count += 1;
                if distinct_count == k {
                    return s.clone();
                }
            }
        }
        
        String::new()
    }
}
