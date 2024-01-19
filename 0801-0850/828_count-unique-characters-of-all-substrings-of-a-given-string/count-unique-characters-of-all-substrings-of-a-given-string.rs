
use std::collections::HashMap;

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let s = s.as_bytes();
        let mut last_pos = vec![-1; 26];
        let mut second_last_pos = vec![-1; 26];
        let mut result = 0;
        let mut unique_count = 0;

        for (i, &ch) in s.iter().enumerate() {
            let index = (ch - b'A') as usize;
            let last = last_pos[index];
            let second_last = second_last_pos[index];
            unique_count += i as i32 - last as i32 - (last as i32 - second_last as i32);
            last_pos[index] = i as i32;
            second_last_pos[index] = last;
            result += unique_count;
        }

        result

    }
}
