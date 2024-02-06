
impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let (a, b) = (pattern.chars().nth(0).unwrap(), pattern.chars().nth(1).unwrap());
        let mut chars = Vec::new();
        
        for c in text.chars() {
            if c == a || c == b {
                chars.push(c);
            }
        }

        if chars.iter().filter(|&&x| x == a).count() >= chars.iter().filter(|&&x| x == b).count() {
            chars.push(b);
        } else {
            chars.insert(0, a);
        }

        let n = chars.len();
        if n < 2 {
            return 0;
        }

        let mut suffix = vec![0; n + 1];
        for i in (0..n).rev() {
            if chars[i] == b {
                suffix[i] += 1 + suffix[i + 1];
            } else {
                suffix[i] = suffix[i + 1];
            }
        }

        let mut answer = 0;
        for (i, &c) in chars.iter().enumerate() {
            if c == a {
                answer += suffix[i + 1];
            }
        }

        answer as i64

    }
}
