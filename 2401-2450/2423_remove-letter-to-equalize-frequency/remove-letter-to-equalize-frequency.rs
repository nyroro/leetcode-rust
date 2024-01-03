
impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        use std::collections::HashMap;

        let mut char_freq = HashMap::new();

        // Step 1: 统计每个字符的频率

        for ch in word.chars() {
            *char_freq.entry(ch).or_insert(0) += 1;
        }

        // Step 3: 对于每个字符，尝试删除它，然后检查剩余字符的频率是否相等

        for (ch, freq) in char_freq.iter() {
            let mut temp_freq = char_freq.clone();
            *temp_freq.get_mut(ch).unwrap() -= 1;
            temp_freq.retain(|_, v| *v > 0);  // Fixed the mutability issue here

            if temp_freq.values().collect::<Vec<_>>().as_slice().windows(2).all(|w| w[0] == w[1]) {
                return true;
            }
        }

        false

    }
}
