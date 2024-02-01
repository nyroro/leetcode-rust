
use std::collections::HashMap;

impl Solution {
    pub fn beautiful_substrings(s: String, k: i32) -> i64 {
        let s = s.as_bytes();
        let n = s.len();
        let mut count = 0;
        let mut prefix = HashMap::new();
        prefix.insert(0, 1);

        let mut v_count = 0;
        let mut c_count = 0;

        for i in 0..n {
            if "aeiou".contains(s[i] as char) {
                v_count += 1;
            } else {
                c_count += 1;
            }

            for j in 0..k {
                let target = v_count - c_count * j;
                count += *prefix.get(&(target % k)).unwrap_or(&0);
            }

            *prefix.entry(v_count - c_count * ((k - 1) % k)).or_insert(0) += 1;
        }

        count

    }
}
