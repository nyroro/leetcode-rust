
impl Solution {
    pub fn largest_word_count(messages: Vec<String>, senders: Vec<String>) -> String {
        use std::collections::HashMap;
        
        let mut word_count: HashMap<String, i32> = HashMap::new();
        
        for (i, sender) in senders.iter().enumerate() {
            let count = messages[i].split_whitespace().count() as i32;
            *word_count.entry(sender.to_string()).or_insert(0) += count;
        }
        
        let max_count = word_count.values().max().unwrap();
        let max_senders: Vec<&String> = word_count.iter()
            .filter(|&(_, v)| v == max_count)
            .map(|(k, _)| k)
            .collect();
        
        max_senders.iter().max().unwrap().to_string()
    }
}
