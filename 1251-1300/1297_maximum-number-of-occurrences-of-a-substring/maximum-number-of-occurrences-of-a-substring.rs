
use std::collections::HashMap;

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        let s = s.as_bytes();
        let mut freq_map: HashMap<&[u8], i32> = HashMap::new();
        let mut max_freq = 0;

        for size in min_size..=max_size {
            let mut unique_chars = 0;
            let mut char_count: HashMap<u8, i32> = HashMap::new();

            for i in 0..s.len() - size as usize + 1 {
                if i == 0 {
                    for j in 0..size as usize {
                        let count = char_count.entry(s[j]).or_insert(0);
                        if *count == 0 {
                            unique_chars += 1;
                        }
                        *count += 1;
                    }
                } else {
                    let prev_char = s[i - 1];
                    let next_char = s[i + size as usize - 1];
                    *char_count.get_mut(&prev_char).unwrap() -= 1;
                    if *char_count.get(&prev_char).unwrap() == 0 {
                        unique_chars -= 1;
                    }
                    let count = char_count.entry(next_char).or_insert(0);
                    if *count == 0 {
                        unique_chars += 1;
                    }
                    *count += 1;
                }

                if unique_chars <= max_letters {
                    let sub = &s[i..i + size as usize];
                    let count = freq_map.entry(sub).or_insert(0);
                    *count += 1;
                    max_freq = max_freq.max(*count);
                }
            }
        }

        max_freq

    }
}
