
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let start_chars: Vec<char> = start.chars().collect();
        let target_chars: Vec<char> = target.chars().collect();
        let n = start.len();
        let (mut i, mut j) = (0, 0);

        while i < n || j < n {
            while i < n && start_chars[i] == '_' {
                i += 1;
            }
            while j < n && target_chars[j] == '_' {
                j += 1;
            }
            if n == i || n == j {
                return i == j;
            }
            if start_chars[i] != target_chars[j] {
                return false;
            }
            if start_chars[i] == 'L' {
                if i < j {
                    return false;
                }
            } else {
                if i > j {
                    return false;
                }
            }
            i += 1;
            j += 1;
        }
        true

    }
}
