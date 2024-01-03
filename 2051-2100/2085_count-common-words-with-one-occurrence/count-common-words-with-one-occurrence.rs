
impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        use std::collections::HashMap;

        // 创建两个HashMap来存储每个字符串在数组中出现的次数

        let mut count1: HashMap<&str, i32> = HashMap::new();
        let mut count2: HashMap<&str, i32> = HashMap::new();

        // 遍历words1数组，统计每个字符串出现的次数

        for word in &words1 {
            *count1.entry(word).or_insert(0) += 1;
        }

        // 遍历words2数组，统计每个字符串出现的次数

        for word in &words2 {
            *count2.entry(word).or_insert(0) += 1;
        }

        // 统计在两个数组中出现且仅出现一次的字符串的数量

        let mut result = 0;
        for (word, count) in count1 {
            if count == 1 && count2.get(word) == Some(&1) {
                result += 1;
            }
        }

        result

    }
}
