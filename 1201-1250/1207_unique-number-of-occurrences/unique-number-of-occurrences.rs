
use std::collections::HashMap;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        // 创建一个 HashMap 来记录每个值的出现次数

        let mut count_map: HashMap<i32, i32> = HashMap::new();
        
        // 遍历数组，统计每个值的出现次数

        for num in arr {
            *count_map.entry(num).or_insert(0) += 1;
        }
        
        // 创建一个 HashSet 来记录出现次数，用于检查是否有重复的出现次数

        let mut occurrences = std::collections::HashSet::new();
        
        // 遍历 HashMap 中的值，检查是否有重复的出现次数

        for count in count_map.values() {
            if !occurrences.insert(*count) {
                return false;
            }
        }
        
        true

    }
}
