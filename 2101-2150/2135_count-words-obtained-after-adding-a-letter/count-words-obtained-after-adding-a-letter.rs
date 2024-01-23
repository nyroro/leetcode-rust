
use std::collections::HashMap;

impl Solution {
    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        let mut table: HashMap<String, i32> = HashMap::new();
        
        for s in start_words {
            let mut sorted_s: Vec<char> = s.chars().collect();
            sorted_s.sort();
            let sorted_s_str: String = sorted_s.iter().collect();
            *table.entry(sorted_s_str).or_insert(0) += 1;
        }

        let mut ret = 0;
        for t in target_words {
            let mut sorted_t: Vec<char> = t.chars().collect();
            sorted_t.sort();
            let sorted_t_str: String = sorted_t.iter().collect();
            for i in 0..t.len() {
                let mut tt = sorted_t_str.clone();
                tt.remove(i);
                if table.contains_key(&tt) {
                    ret += 1;
                    break;
                }
            }
        }
        
        ret

    }
}
