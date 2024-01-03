
use std::collections::HashSet;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut indexes_to_remove: HashSet<usize> = HashSet::new();
        let mut stack: Vec<usize> = Vec::new();

        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => stack.push(i),
                ')' => {
                    if let None = stack.last() {
                        indexes_to_remove.insert(i);
                    } else {
                        stack.pop();
                    }
                }
                _ => {}
            }
        }

        indexes_to_remove.extend(&stack);

        s.chars()
            .enumerate()
            .filter_map(|(i, c)| (!indexes_to_remove.contains(&i)).then(|| c))
            .collect()
    }
}
