
use std::collections::HashMap;



impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut um: HashMap<String, usize> = HashMap::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        let mut empty_index: i32 = -1;
        let mut index_of_palindromes: Vec<usize> = Vec::new();

        for (i, word) in words.iter().enumerate() {
            if word.is_empty() {
                empty_index = i as i32;
                continue;
            }
            if word.chars().rev().collect::<String>() == *word {
                index_of_palindromes.push(i);
            }
            let rev_word = word.chars().rev().collect::<String>();
            um.insert(rev_word, i);
        }

        for (i, word) in words.iter().enumerate() {
            for cut in 0..word.len() {
                if word[cut..].chars().rev().collect::<String>() == word[cut..] {
                    let right = &word[..cut];
                    if let Some(&index) = um.get(right) {
                        if index != i {
                            ans.push(vec![i as i32, index as i32]);
                        }
                    }
                }
                if word[..cut].chars().rev().collect::<String>() == word[..cut] {
                    let left = &word[cut..];
                    if let Some(&index) = um.get(left) {
                        if index != i {
                            ans.push(vec![index as i32, i as i32]);
                        }
                    }
                }
            }
        }

        if empty_index != -1 {
            for &x in &index_of_palindromes {
                ans.push(vec![empty_index, x as i32]);
                ans.push(vec![x as i32, empty_index]);
            }
        }

        ans

    }
}
