
use std::collections::HashMap;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let ra = s.matches('a').count() as i32 - k;
        let rb = s.matches('b').count() as i32 - k;
        let rc = s.matches('c').count() as i32 - k;

        if ra < 0 || rb < 0 || rc < 0 {
            return -1;
        }

        let s: Vec<char> = s.chars().collect();
        let mut hm: HashMap<char, i32> = HashMap::new();
        let (mut left, mut res, mut length) = (0, 0, 0);

        for right in 0..s.len() {
            *hm.entry(s[right]).or_insert(0) += 1;
            length += 1;

            while hm.get(&'a').unwrap_or(&0) > &ra || hm.get(&'b').unwrap_or(&0) > &rb || hm.get(&'c').unwrap_or(&0) > &rc {
                *hm.get_mut(&s[left]).unwrap() -= 1;
                length -= 1;
                left += 1;
            }

            res = res.max(length);
        }

        (s.len() as i32 - res) as i32

    }
}
