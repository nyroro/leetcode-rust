
impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut counts = vec![0; 26];
        let mut max_count = 0;
        let mut max_char = 'a';

        // 统计每个字符的出现次数

        for c in s.chars() {
            let index = (c as u8 - b'a') as usize;
            counts[index] += 1;
            if counts[index] > max_count {
                max_count = counts[index];
                max_char = c;
            }
        }

        // 如果最多出现的字符次数超过字符串长度的一半，则无法满足要求

        if max_count > (s.len() + 1) / 2 {
            return String::from("");
        }

        let mut result = vec![' '; s.len()];
        let mut i = 0;

        // 先填充最多出现的字符

        while counts[(max_char as u8 - b'a') as usize] > 0 {
            result[i] = max_char;
            counts[(max_char as u8 - b'a') as usize] -= 1;
            i += 2;
        }

        // 填充其他字符

        for j in 0..26 {
            let count = counts[j];
            let c = (j as u8 + b'a') as char;
            while counts[j] > 0 {
                if i >= s.len() {
                    i = 1;
                }
                result[i] = c;
                counts[j] -= 1;
                i += 2;
            }
        }

        result.iter().collect()
    }
}
