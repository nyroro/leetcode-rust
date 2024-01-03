
use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut sequence_map: HashMap<&str, i32> = HashMap::new();
        
        let mut i = 0;
        while i + 10 <= s.len() {
            let sequence = &s[i..i+10];
            *sequence_map.entry(sequence).or_insert(0) += 1;
            i += 1;
        }
        
        for (sequence, count) in sequence_map.iter() {
            if *count > 1 {
                result.push(sequence.to_string());
            }
        }
        
        result

    }
}
