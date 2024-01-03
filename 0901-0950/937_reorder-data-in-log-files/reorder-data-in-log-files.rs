
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut letter_logs: Vec<String> = Vec::new();
        let mut digit_logs: Vec<String> = Vec::new();

        for log in logs {
            let words: Vec<&str> = log.split_whitespace().collect();
            if words[1].chars().all(char::is_numeric) {
                digit_logs.push(log);
            } else {
                letter_logs.push(log);
            }
        }

        letter_logs.sort_by(|a, b| {
            let a_words: Vec<&str> = a.split_whitespace().collect();
            let b_words: Vec<&str> = b.split_whitespace().collect();
            let a_content = &a_words[1..].join(" ");
            let b_content = &b_words[1..].join(" ");
            a_content.cmp(b_content).then(a.cmp(b))
        });

        letter_logs.append(&mut digit_logs);
        letter_logs

    }
}
