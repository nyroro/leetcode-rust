
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters_counter: HashMap<char, i32> = HashMap::new();
        
        // 统计 magazine 中每个字母的出现次数

        for letter in magazine.chars() {
            *letters_counter.entry(letter).or_insert(0) += 1;
        }
        
        // 检查 ransomNote 中的每个字母是否可以从 magazine 中构成

        for char in ransom_note.chars() {
            if let Some(count) = letters_counter.get_mut(&char) {
                if *count == 0 {
                    return false;
                }
                *count -= 1;
            } else {
                return false;
            }
        }
        
        true

    }
}
