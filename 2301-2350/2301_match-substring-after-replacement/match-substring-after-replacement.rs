
use std::collections::{HashMap, HashSet};



impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let mut map: HashMap<char, HashSet<char>> = HashMap::new();

        for m in mappings {
            let entry = map.entry(m[0]).or_insert(HashSet::new());
            entry.insert(m[1]);
        }

        let s_chars: Vec<char> = s.chars().collect();
        let sub_chars: Vec<char> = sub.chars().collect();

        let mut i = 0;
        let mut j = 0;

        while j < s_chars.len() {
            if j - i + 1 < sub_chars.len() {
                j += 1;
            } else {
                let mut k = 0;
                while k < sub_chars.len() && (s_chars[i + k] == sub_chars[k] || 
                    (map.contains_key(&sub_chars[k]) && 
                    map[&sub_chars[k]].contains(&s_chars[i + k]))) {
                    k += 1;
                }

                if k == sub_chars.len() {
                    return true;
                }

                i += 1;
            }
        }

        false

    }
}
