
impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let mut count = 0;
        let s_chars: Vec<char> = s.chars().collect();
        
        // 创建一个函数来检查一个子字符串是否美丽

        let is_beautiful = |vowels: i32, consonants: i32| -> bool {
            vowels == consonants && vowels * consonants % k == 0

        };
        
        // 遍历给定字符串的所有可能子字符串，并计算美丽子字符串的数量

        for i in 0..s_chars.len() {
            let mut vowels = 0;
            let mut consonants = 0;
            for j in i..s_chars.len() {
                if "aeiou".contains(s_chars[j]) {
                    vowels += 1;
                } else {
                    consonants += 1;
                }
                if is_beautiful(vowels, consonants) {
                    count += 1;
                }
            }
        }
        
        // 返回美丽子字符串的数量

        count

    }
}
