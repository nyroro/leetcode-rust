
use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut mps: HashMap<(char, i32), i32> = HashMap::new();
        let mut count: i32 = 0;

        for (i, c) in s.chars().enumerate() {
            count = 1;
            *mps.entry((c, count)).or_insert(0) += 1;

            let mut j = i;
            while j + 1 < s.len() {
                if s.chars().nth(j) == s.chars().nth(j + 1) {
                    count += 1;
                    *mps.entry((c, count)).or_insert(0) += 1;
                    j += 1;
                } else {
                    break;
                }
            }
        }

        let mut ans1 = 0;
        for (key, &value) in &mps {
            if value >= 3 {
                ans1 = ans1.max(key.1);
            }
        }

        if ans1 != 0 {
            ans1

        } else {
            -1

        }
    }
}
