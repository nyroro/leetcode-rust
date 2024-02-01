
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        use std::collections::{HashSet, HashMap};
        
        let word_set: HashSet<String> = wordlist.iter().cloned().collect();
        let mut exact_match: HashMap<String, String> = HashMap::new();
        let mut vowel_match: HashMap<String, String> = HashMap::new();
        
        for word in &wordlist {
            let lower_word = word.to_lowercase();
            exact_match.entry(lower_word.clone()).or_insert_with(|| word.clone());
            let devoweled_word = Self::devowel(&lower_word);
            vowel_match.entry(devoweled_word).or_insert_with(|| word.clone());
        }
        
        let mut answer: Vec<String> = Vec::new();
        
        for query in queries {
            if word_set.contains(&query) {
                answer.push(query.clone());
            } else {
                let lower_query = query.to_lowercase();
                if let Some(&original) = exact_match.get(&lower_query) {
                    answer.push(original);
                } else if let Some(&original) = vowel_match.get(&Self::devowel(&lower_query)) {
                    answer.push(original);
                } else {
                    answer.push("".to_string());
                }
            }
        }
        
        answer

    }
    
    fn devowel(s: &str) -> String {
        s.chars().map(|c| match c {
            'a' | 'e' | 'i' | 'o' | 'u' => '_',
            _ => c

        }).collect()
    }
}
