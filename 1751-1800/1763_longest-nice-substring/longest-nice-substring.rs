
impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        let mut result = String::new();
        for i in 0..s.len() {
            for j in (i + 1)..s.len() {
                let sub = &s[i..=j];
                let mut nice = true;
                let mut set = std::collections::HashSet::new();
                for c in sub.chars() {
                    set.insert(c.to_ascii_lowercase());
                }
                for c in sub.chars() {
                    if !set.contains(&c.to_ascii_lowercase()) {
                        if c.is_ascii_lowercase() {
                            if !set.contains(&c.to_ascii_uppercase()) {
                                nice = false;
                                break;
                            }
                        } else {
                            if !set.contains(&c.to_ascii_lowercase()) {
                                nice = false;
                                break;
                            }
                        }
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
