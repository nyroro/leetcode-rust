
use std::collections::HashMap;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut word_map: HashMap<u32, i32> = HashMap::new();

        // Step 1: Create a bitmask for each word and store it in the word_map

        for word in words.iter() {
            let mut bitmask: u32 = 0;
            for ch in word.chars() {
                bitmask |= 1 << (ch as u32 - 'a' as u32);
            }
            *word_map.entry(bitmask).or_insert(0) += 1;
        }

        let mut result = Vec::new();
        // Step 2-3: Generate subsets for each puzzle and count the valid words

        for puzzle in puzzles.iter() {
            let mut count = 0;
            let mut first_char = 1 << (puzzle.chars().next().unwrap() as u32 - 'a' as u32);
            let mut bitmask = 0;

            for ch in puzzle.chars() {
                bitmask |= 1 << (ch as u32 - 'a' as u32);
            }

            let mut subset = bitmask;
            loop {
                if subset & first_char > 0 {
                    count += word_map.get(&subset).unwrap_or(&0);
                }
                if subset == 0 {
                    break;
                }
                subset = (subset - 1) & bitmask;
            }
            result.push(count);
        }
        result

    }
}
