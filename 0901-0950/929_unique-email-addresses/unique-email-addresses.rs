
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let mut unique_emails = HashSet::new();

        for email in emails {
            let mut parts = email.split('@');
            let local_name = parts.next().unwrap();
            let domain_name = parts.next().unwrap();

            let mut filtered_local_name = String::new();
            for c in local_name.chars() {
                if c == '.' {
                    continue;
                } else if c == '+' {
                    break;
                }
                filtered_local_name.push(c);
            }

            let formatted_email = format!("{}@{}", filtered_local_name, domain_name);
            unique_emails.insert(formatted_email);
        }

        unique_emails.len() as i32

    }
}
