
use std::collections::HashMap;

struct SubstringEndpoints {
    left: i32,
    right: i32,
}

impl Solution {
    pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut seen: HashMap<i32, SubstringEndpoints> = HashMap::new();
        let s_chars: Vec<char> = s.chars().collect();

        for (i, &ch) in s_chars.iter().enumerate() {
            if ch == '1' {
                let mut val = 0;
                for j in i..std::cmp::min(s_chars.len(), i + 30) {
                    val <<= 1;
                    if s_chars[j] == '1' {
                        val ^= 1;
                    }
                    seen.entry(val).or_insert(SubstringEndpoints { left: i as i32, right: j as i32 });
                }
            } else {
                seen.entry(0).or_insert(SubstringEndpoints { left: i as i32, right: i as i32 });
            }
        }

        let mut result: Vec<Vec<i32>> = Vec::new();
        for query in queries {
            let xor_val = query[0] ^ query[1];
            if let Some(substring) = seen.get(&xor_val) {
                result.push(vec![substring.left, substring.right]);
            } else {
                result.push(vec![-1, -1]);
            }
        }

        result

    }
}
