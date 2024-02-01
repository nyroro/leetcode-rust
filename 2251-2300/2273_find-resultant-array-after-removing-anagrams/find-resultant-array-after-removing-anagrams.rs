
impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut prev_anagram = String::new();
        
        for word in words {
            if !is_anagram(&prev_anagram, &word) {
                result.push(word);
                prev_anagram = word;
            }
        }
        
        result

    }
    
    fn is_anagram(word1: &String, word2: &String) -> bool {
        let mut chars1: Vec<char> = word1.chars().collect();
        let mut chars2: Vec<char> = word2.chars().collect();
        chars1.sort();
        chars2.sort();
        chars1 == chars2

    }
}
