
use std::collections::HashMap;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut char_positions: HashMap<char, (usize, usize)> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let mut count = 0;

        // 记录每个字符的出现位置

        for (i, &c) in chars.iter().enumerate() {
            let positions = char_positions.entry(c).or_insert((usize::MAX, usize::MIN));
            positions.0 = positions.0.min(i);
            positions.1 = positions.1.max(i);
        }

        // 遍历字符串，计算回文子序列数量

        for (_, &(start, end)) in char_positions.iter() {
            let mut unique_chars: Vec<char> = vec![];

            // 统计字符种类数量

            for i in (start + 1)..end {
                if !unique_chars.contains(&chars[i]) {
                    unique_chars.push(chars[i]);
                }
            }

            count += unique_chars.len();
        }

        count as i32

    }
}
