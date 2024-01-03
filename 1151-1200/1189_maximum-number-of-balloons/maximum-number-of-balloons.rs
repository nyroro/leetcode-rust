
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        // 创建一个HashMap来存储'b', 'a', 'l', 'o', 'n'这五个字母的出现次数

        let mut char_count = std::collections::HashMap::new();
        
        // 遍历给定的字符串，统计'b', 'a', 'l', 'o', 'n'这五个字母的出现次数

        for c in text.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }
        
        // 计算可以组成多少个单词"balloon"
        let mut result = std::i32::MAX;
        for &c in ['b', 'a', 'l', 'o', 'n'].iter() {
            let count = match c {
                'l' | 'o' => char_count.get(&c).map_or(0, |&x| x / 2),
                _ => char_count.get(&c).map_or(0, |&x| x),
            };
            result = result.min(count);
        }
        
        result

    }
}
