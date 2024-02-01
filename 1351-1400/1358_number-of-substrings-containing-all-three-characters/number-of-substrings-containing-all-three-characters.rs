
use std::collections::HashMap;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut count = 0;
        let mut freq: HashMap<char, i32> = HashMap::new();
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut left = 0;
        let mut right = 0;

        while right < n {
            *freq.entry(chars[right]).or_insert(0) += 1;

            while freq.get(&'a').unwrap_or(&0) > &0 && freq.get(&'b').unwrap_or(&0) > &0 && freq.get(&'c').unwrap_or(&0) > &0 {
                count += (n - right) as i32;
                *freq.entry(chars[left]).or_insert(0) -= 1;
                left += 1;
            }

            right += 1;
        }

        count

    }
}
