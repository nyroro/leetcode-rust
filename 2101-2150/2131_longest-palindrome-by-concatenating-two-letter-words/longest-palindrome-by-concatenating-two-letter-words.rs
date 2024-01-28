
use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut table: HashMap<String, i32> = HashMap::new();
        let mut ret = 0;

        for t in words {
            let count = table.entry(t.clone()).or_insert(0);
            if *count>0 {
                *count -= 1;
                ret += 2;
            } else {
                let rt = t.chars().rev().collect::<String>();
                
                let count = table.entry(rt).or_insert(0);
                *count += 1;
            }
        }

        for (key, count) in table.iter() {
            if *key == key.chars().rev().collect::<String>() && *count > 0 {
                ret += 1;
                break;
            }
        }

        ret * 2

    }
}
