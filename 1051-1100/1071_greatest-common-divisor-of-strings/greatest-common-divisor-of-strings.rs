
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        // Check if the lengths of the strings are equal

        if str1.len() != str2.len() {
            return String::new();
        }

        // Check if str2 divides str1

        if str1.contains(&str2) {
            let mut temp = str1.clone();
            while temp.contains(&str2) {
                temp = temp.replace(&str2, "");
            }
            if temp.is_empty() {
                return str2;
            }
        }

        // Check if str1 divides str2

        if str2.contains(&str1) {
            let mut temp = str2.clone();
            while temp.contains(&str1) {
                temp = temp.replace(&str1, "");
            }
            if temp.is_empty() {
                return str1;
            }
        }

        String::new()
    }
}
