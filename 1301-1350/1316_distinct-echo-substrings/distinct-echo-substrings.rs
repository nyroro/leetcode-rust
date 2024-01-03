
use std::collections::HashSet;

impl Solution {
    pub fn distinct_echo_substrings(text: String) -> i32 {
        let mut substrings = HashSet::new();
        let text_bytes = text.as_bytes();

        for i in 0..text.len() {
            for j in (i + 1)..text.len() {
                let len = j - i;
                if j + len <= text.len() {
                    let first = &text_bytes[i..(i + len)];
                    let second = &text_bytes[j..(j + len)];
                    if first == second {
                        substrings.insert(String::from_utf8_lossy(first).to_string());
                    }
                }
            }
        }

        substrings.len() as i32

    }
}
