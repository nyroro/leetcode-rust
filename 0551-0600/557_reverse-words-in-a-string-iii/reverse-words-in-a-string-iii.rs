
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result = String::new();
        let words: Vec<&str> = s.split(' ').collect();
        
        for word in words {
            let reversed_word: String = word.chars().rev().collect();
            result.push_str(&reversed_word);
            result.push(' ');
        }
        
        result.pop(); // 移除最后一个多余的空格

        result

    }
}
