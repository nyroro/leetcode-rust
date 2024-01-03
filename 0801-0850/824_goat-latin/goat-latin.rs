
impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
        let words: Vec<&str> = sentence.split_whitespace().collect();
        let mut result = String::new();
        
        for (i, word) in words.iter().enumerate() {
            let mut new_word = String::new();
            let first_char = word.chars().next().unwrap();
            
            if vowels.contains(&first_char.to_ascii_lowercase()) {
                new_word.push_str(word);
            } else {
                new_word.push_str(&word[1..]);
                new_word.push(first_char);
            }
            
            new_word.push_str("ma");
            new_word.push_str(&"a".repeat(i + 1));
            
            result.push_str(&new_word);
            result.push(' ');
        }
        
        result.trim_end().to_string()
    }
}
