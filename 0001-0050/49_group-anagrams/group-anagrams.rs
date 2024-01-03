
use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();
        
        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort();
            let sorted_str: String = chars.into_iter().collect();
            
            if let Some(group) = anagrams.get_mut(&sorted_str) {
                group.push(str);
            } else {
                anagrams.insert(sorted_str, vec![str]);
            }
        }
        
        anagrams.into_iter().map(|(_, group)| group).collect()
    }
}
