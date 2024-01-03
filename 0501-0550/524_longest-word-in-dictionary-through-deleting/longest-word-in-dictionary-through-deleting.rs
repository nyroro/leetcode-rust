
impl Solution {
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut longest_word = String::new();
        
        for word in dictionary {
            let mut i = 0;
            let mut j = 0;
            
            while i < s.len() && j < word.len() {
                if s.chars().nth(i) == word.chars().nth(j) {
                    j += 1;
                }
                i += 1;
            }
            
            if j == word.len() && (word.len() > longest_word.len() || (word.len() == longest_word.len() && word < longest_word)) {
                longest_word = word;
            }
        }
        
        longest_word

    }
}
