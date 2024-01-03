
use std::collections::HashMap;

impl Solution {
    pub fn get_words_in_longest_subsequence(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut word_indices: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &group) in groups.iter().enumerate() {
            word_indices.entry(group).or_insert(vec![]).push(i);
        }
        
        let mut dp: Vec<usize> = vec![1; n as usize];
        let mut max_length = 1;
        let mut max_index = 0;
        
        for i in 0..n as usize {
            for j in 0..i {
                if words[i].len() == words[j].len() && groups[i] != groups[j] {
                    let mut diff_count = 0;
                    for (c1, c2) in words[i].chars().zip(words[j].chars()) {
                        if c1 != c2 {
                            diff_count += 1;
                        }
                    }
                    if diff_count == 1 {
                        if dp[j] + 1 > dp[i] {
                            dp[i] = dp[j] + 1;
                            if dp[i] > max_length {
                                max_length = dp[i];
                                max_index = i;
                            }
                        }
                    }
                }
            }
        }
        
        let mut result: Vec<String> = Vec::new();
        for _ in 0..max_length {
            result.push(words[max_index].clone());
            let mut found = false;
            for j in (0..max_index).rev() {
                if dp[j] + 1 == dp[max_index] && words[j].len() == words[max_index].len() && groups[j] != groups[max_index] {
                    let mut diff_count = 0;
                    for (c1, c2) in words[j].chars().zip(words[max_index].chars()) {
                        if c1 != c2 {
                            diff_count += 1;
                        }
                    }
                    if diff_count == 1 {
                        max_index = j;
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                break;
            }
        }
        
        result.reverse();
        result

    }
}
