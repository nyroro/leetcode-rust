
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        let mut result = Vec::new();
        
        for word in words {
            if Self::is_match(&word, &pattern) {
                result.push(word);
            }
        }
        
        result

    }
    
    fn is_match(word: &str, pattern: &str) -> bool {
        if word.len() != pattern.len() {
            return false;
        }
        
        let mut map = HashMap::new();
        let mut used = HashSet::new();
        
        for (w, p) in word.chars().zip(pattern.chars()) {
            if let Some(&mapped) = map.get(&p) {
                if mapped != w {
                    return false;
                }
            } else {
                if used.contains(&w) {
                    return false;
                }
                
                map.insert(p, w);
                used.insert(w);
            }
        }
        
        true

    }
}
