
use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        
        let mut s_to_t = HashMap::new();
        let mut t_to_s = HashMap::new();
        
        for (i, &c1) in s.as_bytes().iter().enumerate() {
            let c2 = t.as_bytes()[i];
            
            if let Some(&mapped_c2) = s_to_t.get(&c1) {
                if mapped_c2 != c2 {
                    return false;
                }
            } else {
                s_to_t.insert(c1, c2);
            }
            
            if let Some(&mapped_c1) = t_to_s.get(&c2) {
                if mapped_c1 != c1 {
                    return false;
                }
            } else {
                t_to_s.insert(c2, c1);
            }
        }
        
        true

    }
}
