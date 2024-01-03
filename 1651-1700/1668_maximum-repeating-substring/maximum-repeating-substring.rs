
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut max_repeating = 0;
        let word_len = word.len();
        let sequence_len = sequence.len();
        
        for i in 0..sequence_len {
            let mut k = 0;
            let mut j = i;
            
            while j + word_len <= sequence_len && &sequence[j..j+word_len] == &word[..] {
                k += 1;
                j += word_len;
            }
            
            max_repeating = max_repeating.max(k);
        }
        
        max_repeating

    }
}
