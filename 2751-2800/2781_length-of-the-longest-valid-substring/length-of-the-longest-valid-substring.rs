
use std::collections::HashSet;

impl Solution {
    pub fn longest_valid_substring(word: String, forbidden: Vec<String>) -> i32 {
        let forbidden_set: HashSet<String> = forbidden.into_iter().collect();
        let mut res = 0;
        let mut right = word.len() - 1;

        for left in (0..word.len()).rev() {
            for k in left..(left + 10).min(right + 1) {
                if forbidden_set.contains(&word[left..=k]) {
                    right = k as usize - 1;
                    break;
                }
            }
            res = res.max((right - left + 1) as i32);
        }

        res

    }
}
