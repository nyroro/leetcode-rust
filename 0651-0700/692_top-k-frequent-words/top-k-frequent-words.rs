
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        // Step 1: Create a hashmap to count the frequency of each word

        let mut word_count: HashMap<String, i32> = HashMap::new();
        
        // Step 2: Count the frequency of each word

        for word in words {
            let count = word_count.entry(word).or_insert(0);
            *count += 1;
        }
        
        // Step 3: Convert the hashmap to a vector of tuples

        let mut word_freq: Vec<(String, i32)> = word_count.into_iter().collect();
        
        // Step 4: Sort the vector based on frequency and lexicographical order

        word_freq.sort_by(|a, b| {
            if a.1 != b.1 {
                b.1.cmp(&a.1)
            } else {
                a.0.cmp(&b.0)
            }
        });
        
        // Step 5: Get the top k frequent words

        let result: Vec<String> = word_freq.into_iter().take(k as usize).map(|(word, _)| word).collect();
        
        // Step 6: Return the result

        result

    }
}
