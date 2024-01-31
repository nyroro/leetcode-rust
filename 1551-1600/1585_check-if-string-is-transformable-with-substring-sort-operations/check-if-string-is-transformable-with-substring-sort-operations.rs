
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        let mut indexes: HashMap<char, Vec<usize>> = HashMap::new();

        for (i, &ch) in s_chars.iter().enumerate() {
            indexes.entry(ch).or_insert(Vec::new()).push(i);
        }

        let unique_t_values: HashSet<char> = t_chars.iter().cloned().collect();

        for &ch in t_chars.iter() {
            if !indexes.contains_key(&ch) {
                return false;
            }

            for prev_ch in '0'..ch {
                if unique_t_values.contains(&prev_ch) {
                    if let Some(prev_positions) = indexes.get(&prev_ch) {
                        if let Some(curr_positions) = indexes.get(&ch) {
                            if let Some(prev_pos) = prev_positions.iter().next() {
                                if let Some(curr_pos) = curr_positions.iter().next() {
                                    if prev_pos < curr_pos {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(curr_positions) = indexes.get_mut(&ch) {
                if let Some(curr_pos) = curr_positions.iter().next().cloned() {
                    curr_positions.remove(curr_positions.iter().position(|&x| x == curr_pos).unwrap());
                    if curr_positions.is_empty() {
                        indexes.remove(&ch);
                    }
                }
            }
        }

        true

    }
}
