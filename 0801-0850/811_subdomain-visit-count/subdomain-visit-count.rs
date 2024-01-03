
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut domain_counts: HashMap<String, i32> = HashMap::new();

        for cpdomain in cpdomains {
            let parts: Vec<&str> = cpdomain.split_whitespace().collect();
            let count = parts[0].parse::<i32>().unwrap();
            let domain = parts[1];

            let subdomains: Vec<&str> = domain.split('.').collect();
            let mut current_domain = String::new();

            for i in (0..subdomains.len()).rev() {
                if !current_domain.is_empty() {
                    current_domain.insert(0, '.');
                }
                current_domain.insert_str(0, subdomains[i]);

                *domain_counts.entry(current_domain.clone()).or_insert(0) += count;
            }
        }

        let mut result: Vec<String> = Vec::new();

        for (domain, count) in domain_counts {
            let cpdomain = format!("{} {}", count, domain);
            result.push(cpdomain);
        }

        result

    }
}
