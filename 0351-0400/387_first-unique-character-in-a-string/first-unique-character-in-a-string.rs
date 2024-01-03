
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut char_count = [0; 26]; // 用于记录每个字符出现的次数

        let s_bytes = s.as_bytes(); // 将字符串转换为字节数组


        // 统计每个字符出现的次数

        for &ch in s_bytes.iter() {
            char_count[(ch - b'a') as usize] += 1;
        }

        // 找到第一个不重复的字符并返回其索引

        for (i, &ch) in s_bytes.iter().enumerate() {
            if char_count[(ch - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        -1 // 如果没有不重复的字符，则返回-1

    }
}
