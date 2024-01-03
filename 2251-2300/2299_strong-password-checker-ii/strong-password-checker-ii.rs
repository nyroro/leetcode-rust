
impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        // 检查密码长度是否至少为8个字符

        if password.len() < 8 {
            return false;
        }
        
        // 检查密码是否包含至少一个小写字母、一个大写字母和一个数字

        let mut has_lowercase = false;
        let mut has_uppercase = false;
        let mut has_digit = false;
        for c in password.chars() {
            if c.is_ascii_lowercase() {
                has_lowercase = true;
            } else if c.is_ascii_uppercase() {
                has_uppercase = true;
            } else if c.is_ascii_digit() {
                has_digit = true;
            }
        }
        if !has_lowercase || !has_uppercase || !has_digit {
            return false;
        }
        
        // 检查密码是否包含至少一个特殊字符

        let special_chars = "!@#$%^&*()-+";
        let mut has_special_char = false;
        for c in password.chars() {
            if special_chars.contains(c) {
                has_special_char = true;
                break;
            }
        }
        if !has_special_char {
            return false;
        }
        
        // 检查密码是否包含相邻重复字符

        for i in 0..password.len() - 1 {
            if password[i..=i] == password[i + 1..=i + 1] {
                return false;
            }
        }
        
        // 如果密码满足所有要求，则返回true

        true

    }
}
