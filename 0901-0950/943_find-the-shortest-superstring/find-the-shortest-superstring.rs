
use std::collections::HashMap;



impl Solution {
    pub fn shortest_superstring(words: Vec<String>) -> String {
        fn get_min_suffix(w1: &str, w2: &str) -> String {
            let n = std::cmp::min(w1.len(), w2.len());
            for i in (1..=n).rev() {
                if &w1[w1.len() - i..] == &w2[..i] {
                    return w2[i..].to_string();
                }
            }
            w2.to_string()
        }

        let n = words.len();
        let mut suffix = vec![vec!["".to_string(); n]; n];
        for i in 0..n {
            for j in 0..n {
                suffix[i][j] = get_min_suffix(&words[i], &words[j]);
            }
        }

        let mut dp = vec![vec!["".to_string(); n]; 1 << n];
        for i in 1..(1 << n) {
            let indexes: Vec<usize> = (0..n).filter(|&j| i & (1 << j) != 0).collect();
            for &j in &indexes {
                let i2 = i & !(1 << j);
                let mut strs = vec![];
                for &j2 in &indexes {
                    if j2 != j {
                        strs.push(dp[i2][j2].clone() + &suffix[j2][j]);
                    }
                }
                dp[i][j] = strs.into_iter().min_by_key(|s| s.len()).unwrap_or_else(|| words[j].clone());
            }
        }

        dp[(1 << n) - 1].iter().min_by_key(|s| s.len()).unwrap().clone()
    }
}
