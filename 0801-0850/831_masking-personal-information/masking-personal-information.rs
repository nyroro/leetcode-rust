
impl Solution {
    pub fn mask_pii(s: String) -> String {
        if s.contains('@') {
            // 处理邮箱地址

            let mut result = String::new();
            let email_parts: Vec<&str> = s.split('@').collect();
            let name = email_parts[0].to_lowercase();
            let domain = email_parts[1].to_lowercase();
            result.push(name.chars().next().unwrap());
            result.push_str("*****");
            result.push(name.chars().last().unwrap());
            result.push('@');
            result.push_str(&domain);
            result

        } else {
            // 处理电话号码

            let mut result = String::new();
            let digits: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
            let local_number = digits.iter().skip(digits.len() - 10).collect::<String>();
            let country_code_len = digits.len() - 10;
            if country_code_len > 0 {
                result.push('+');
                result.push_str(&"*".repeat(country_code_len));
                result.push('-');
            }
            result.push_str("***-***-");
            result.push_str(&local_number.chars().skip(local_number.len() - 4).collect::<String>());
            result

        }
    }
}
