
impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let n = pattern.len();
        let mut result = String::new();
        let mut current_min = 1;
        let mut current_max = n + 1;

        for ch in pattern.chars() {
            if ch == 'I' {
                result.push_str(current_min.to_string().as_str());
                current_min += 1;
            } else {
                result.push_str(current_max.to_string().as_str());
                current_max -= 1;
            }
        }

        result.push_str(current_min.to_string().as_str());
        result

    }
}
