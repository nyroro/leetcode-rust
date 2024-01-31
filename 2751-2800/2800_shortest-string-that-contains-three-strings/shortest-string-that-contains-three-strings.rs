


impl Solution {
    pub fn minimum_string(a: String, b: String, c: String) -> String {
        // Function to find the longest suffix of s1 that is a prefix of s2

        fn longest_common_suffix(s1: &str, s2: &str) -> String {
            if s1.contains(s2) {
                return String::new();
            }
            for i in (0..s2.len()).rev() {
                let mut p1 = i as i32;  // Change p1 to a signed integer type

                let mut p2 = s1.len() as i32 - 1;  // Change p2 to a signed integer type

                while p2 >= 0 && p1 >= 0 && s1.chars().nth(p2 as usize) == s2.chars().nth(p1 as usize) {
                    p2 -= 1;
                    p1 -= 1;
                }
                if p1 == -1 {
                    return s2.chars().skip(i + 1).collect();
                }
            }
            s2.to_string()
        }

        // Concatenate the strings based on the longest common suffix

        fn concatenate_strings(a: &str, b: &str, c: &str) -> String {
            let mut result = a.to_string();
            result.push_str(&longest_common_suffix(&result, &b));
            result.push_str(&longest_common_suffix(&result, &c));
            result

        }

        let possibilities = [
            concatenate_strings(&a, &b, &c),
            concatenate_strings(&a, &c, &b),
            concatenate_strings(&b, &a, &c),
            concatenate_strings(&b, &c, &a),
            concatenate_strings(&c, &a, &b),
            concatenate_strings(&c, &b, &a),
        ];

        // Find the lexicographically smallest string among the possibilities

        let mut min_string = possibilities[0].clone();
        for s in &possibilities {
            if s.len() < min_string.len() || (s.len() == min_string.len() && s < &min_string) {
                min_string = s.clone();
            }
        }

        min_string

    }
}
