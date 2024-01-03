
impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        for i in 0..words.len() {
            for j in (i + 1)..words.len() {
                if Solution::are_similar(&words[i], &words[j]) {
                    count += 1;
                }
            }
        }
        count

    }
    
    fn are_similar(word1: &String, word2: &String) -> bool {
        use std::collections::HashSet;
        let set1: HashSet<char> = word1.chars().collect();
        let set2: HashSet<char> = word2.chars().collect();
        set1 == set2

    }
}
