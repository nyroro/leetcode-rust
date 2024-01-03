
use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let word_set: HashSet<String> = word_dict.into_iter().collect();
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[0] = true;

        for i in 1..=n {
            for j in 0..i {
                if dp[j] && word_set.contains(&s[j..i]) {
                    dp[i] = true;
                    break;
                }
            }
        }

        let mut result = Vec::new();
        if dp[n] {
            let mut path = Vec::new();
            Solution::word_break_helper(&s, &word_set, &mut result, &mut path, n);
        }

        result

    }

    fn word_break_helper(
        s: &str,
        word_set: &HashSet<String>,
        result: &mut Vec<String>,
        path: &mut Vec<String>,
        end: usize,
    ) {
        if end == 0 {
            result.push(path.iter().rev().cloned().collect::<Vec<String>>().join(" "));
            return;
        }

        for i in (0..end).rev() {
            let word = &s[i..end];
            if word_set.contains(word) {
                path.push(word.to_string());
                Solution::word_break_helper(s, word_set, result, path, i);
                path.pop();
            }
        }
    }
}
