
use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let word_set: HashSet<String> = word_list.into_iter().collect();
        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        let mut visited: HashSet<String> = HashSet::new();
        
        queue.push_back((begin_word.clone(), 1)); // 克隆begin_word并放入队列

        visited.insert(begin_word);
        
        while let Some((word, length)) = queue.pop_front() {
            if word == end_word {
                return length;
            }
            
            for i in 0..word.len() {
                let mut new_word = word.clone().into_bytes();
                for j in b'a'..=b'z' {
                    new_word[i] = j;
                    let neighbor = String::from_utf8(new_word.clone()).unwrap();
                    if word_set.contains(&neighbor) && !visited.contains(&neighbor) {
                        queue.push_back((neighbor.clone(), length + 1));
                        visited.insert(neighbor);
                    }
                }
            }
        }
        
        0

    }
}
