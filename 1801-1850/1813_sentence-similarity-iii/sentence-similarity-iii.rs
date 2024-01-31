


impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let words1: Vec<&str> = sentence1.split_whitespace().collect();
        let words2: Vec<&str> = sentence2.split_whitespace().collect();
        
        let (shorter, longer) = if words1.len() <= words2.len() {
            (&words1, &words2)
        } else {
            (&words2, &words1)
        };

        let mut i = 0;
        while i < shorter.len() && shorter[i] == longer[i] {
            i += 1;
        }
        while i < shorter.len() && shorter[i] == longer[longer.len() - shorter.len() + i] {
            i += 1;
        }

        i == shorter.len()
    }
}
