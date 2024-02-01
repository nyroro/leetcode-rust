
impl Solution {
    pub fn make_largest_special(s: String) -> String {
        // 找到所有的特殊子字符串

        let mut special_strings = Self::find_special_strings(&s);

        // 对特殊子字符串进行递归处理

        for s in &mut special_strings {
            let inner = s[1..s.len() - 1].to_string();
            let sorted_inner = Self::make_largest_special(inner);
            s.clear();
            s.push('1');
            s.push_str(&sorted_inner);
            s.push('0');
        }

        // 按照字典序排序特殊子字符串

        special_strings.sort();

        // 将特殊子字符串连接起来得到最终结果

        special_strings.join("")
    }

    // 找到所有的特殊子字符串

    fn find_special_strings(s: &str) -> Vec<String> {
        let mut count = 0;
        let mut start = 0;
        let mut result = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c == '1' {
                count += 1;
            } else {
                count -= 1;
            }

            if count == 0 {
                let special_string = s[start..=i].to_string();
                result.push(special_string);
                start = i + 1;
            }
        }

        result

    }
}
