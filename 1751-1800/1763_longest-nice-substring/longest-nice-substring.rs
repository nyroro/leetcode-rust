
impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let mut result = String::new();
        for i in 0..s.len() {
            for j in (i + 1)..s.len() {
                let sub = &s[i..=j];
                let mut nice = true;
                for c in sub.chars() {
                    let lower = c.to_ascii_lowercase();
                    let upper = c.to_ascii_uppercase();
                    if sub.matches(lower).count() > 0 && sub.matches(upper).count() > 0 {
                        continue;
                    } else {
                        nice = false;
                        break;
                    }
                }
                if nice && sub.len() > result.len() {
                    result = sub.to_string();
                }
            }
        }
        result

    }
}
