
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut char_count = vec![0; 26]; // 创建一个长度为26的数组，用于统计每个字符出现的次数

        let n = words.len(); // 获取数组的长度


        // 统计每个字符出现的次数

        for word in &words {
            for ch in word.chars() {
                let idx = (ch as u8 - b'a') as usize; // 计算字符在数组中的索引

                char_count[idx] += 1; // 更新字符出现次数

            }
        }

        // 检查每个字符出现的次数是否能够被数组的长度整除

        for count in char_count {
            if count % n != 0 {
                return false; // 如果有任意一个字符的出现次数不能被数组长度整除，返回false

            }
        }

        return true; // 所有字符的出现次数都能被数组长度整除，返回true

    }
}
