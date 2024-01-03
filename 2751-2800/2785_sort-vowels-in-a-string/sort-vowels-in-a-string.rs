
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let mut vowels: Vec<char> = Vec::new();
        
        // 判断一个字符是否为元音字母

        fn is_vowel(c: char) -> bool {
            let lower_c = c.to_ascii_lowercase();
            lower_c == 'a' || lower_c == 'e' || lower_c == 'i' || lower_c == 'o' || lower_c == 'u'
        }
        
        // 遍历字符数组，将元音字母添加到新数组中，保持辅音字母在原位置

        for i in 0..chars.len() {
            if is_vowel(chars[i]) {
                vowels.push(chars[i]);
                chars[i] = ' ';
            }
        }
        
        // 对新数组中的元素按照ASCII值进行排序

        vowels.sort();
        
        // 将排序后的元素与原字符数组进行合并

        let mut result = String::new();
        let mut vowel_index = 0;
        for i in 0..chars.len() {
            if chars[i] == ' ' {
                result.push(vowels[vowel_index]);
                vowel_index += 1;
            } else {
                result.push(chars[i]);
            }
        }
        
        result

    }
}
