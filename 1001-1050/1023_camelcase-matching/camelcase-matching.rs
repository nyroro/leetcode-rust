
impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut answer = vec![false; queries.len()];

        for (i, query) in queries.iter().enumerate() {
            let mut pattern_iter = pattern.chars().peekable();
            let mut match_found = true;

            for c in query.chars() {
                if pattern_iter.peek().is_none() || c != *pattern_iter.peek().unwrap() {
                    if c.is_ascii_uppercase() {
                        match_found = false;
                        break;
                    }
                } else {
                    pattern_iter.next();
                }
            }

            if match_found && pattern_iter.peek().is_none() {
                answer[i] = true;
            }
        }

        answer

    }
}
