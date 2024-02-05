
use std::collections::HashMap;



impl Solution {
    pub fn minimize_concatenated_length(words: Vec<String>) -> i32 {
        if words.len() == 1 {
            return words[0].len() as i32;
        }

        let mut dp: HashMap<(usize, char, char), i32> = HashMap::new();

        fn helper(i: usize, front: char, back: char, words: &Vec<String>, dp: &mut HashMap<(usize, char, char), i32>) -> i32 {
            if i == words.len() {
                return 0;
            }
            if let Some(&res) = dp.get(&(i, front, back)) {
                return res;
            }

            let a = helper(i + 1, front, words[i].chars().last().unwrap(), words, dp) - if back == words[i].chars().next().unwrap() { 1 } else { 0 };
            let b = helper(i + 1, words[i].chars().next().unwrap(), back, words, dp) - if front == words[i].chars().last().unwrap() { 1 } else { 0 };

            let result = std::cmp::min(a, b) + words[i].len() as i32;
            dp.insert((i, front, back), result);
            result

        }

        helper(1, words[0].chars().next().unwrap(), words[0].chars().last().unwrap(), &words, &mut dp) + words[0].len() as i32

    }
}
