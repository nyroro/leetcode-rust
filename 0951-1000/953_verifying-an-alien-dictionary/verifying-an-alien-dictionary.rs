
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        // Create a mapping of characters to their indices in the order

        let mut map = vec![0; 26];
        for (i, c) in order.chars().enumerate() {
            map[(c as u8 - b'a') as usize] = i;
        }
        
        // Iterate through the words and compare adjacent pairs

        for i in 0..words.len() - 1 {
            if !Self::is_sorted_in_alien_language(&words[i], &words[i + 1], &map) {
                return false;
            }
        }
        
        true

    }
    
    // Helper function to check if two words are sorted in alien language order

    fn is_sorted_in_alien_language(word1: &String, word2: &String, map: &Vec<usize>) -> bool {
        let chars1: Vec<char> = word1.chars().collect();
        let chars2: Vec<char> = word2.chars().collect();
        
        let mut i = 0;
        while i < chars1.len() && i < chars2.len() {
            let index1 = map[(chars1[i] as u8 - b'a') as usize];
            let index2 = map[(chars2[i] as u8 - b'a') as usize];
            if index1 < index2 {
                return true;
            } else if index1 > index2 {
                return false;
            }
            i += 1;
        }
        
        chars1.len() <= chars2.len()
    }
}
