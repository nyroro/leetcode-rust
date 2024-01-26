
use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let s_chars: Vec<char> = s.chars().collect();
        let mut char_counts = HashMap::new();
        
        for &ch in &s_chars {
            *char_counts.entry(ch).or_insert(0) += 1;
        }
        
        let mut valid_chars: Vec<char> = char_counts.iter()
            .filter(|&(_, &count)| count >= k as usize)
            .map(|(&ch, _)| ch)
            .collect();
        
        let mut queue: Vec<String> = vec!["".to_string()];
        let mut ret = String::new();
        
        let mut qi = 0;
        
        let valid = |tx: &str| -> bool {
            let tx = tx.repeat(k as usize);
            if tx.len() > s.len() {
                return false;
            }
            let mut i = 0;
            for &t in &s_chars {
                if t == tx.as_bytes()[i] as char {
                    i += 1;
                }
                if i == tx.len() {
                    break;
                }
            }
            i == tx.len()
        };
        
        while qi < queue.len() {
            let x = queue[qi].clone();
            qi += 1;
            for &c in &valid_chars {
                let tx = format!("{}{}", x, c);
                if valid(&tx) {
                    queue.push(tx.clone());
                    if tx.len() > ret.len() || (tx.len() == ret.len() && tx > ret) {
                        ret = tx;
                    }
                }
            }
        }
        
        ret

    }
}
