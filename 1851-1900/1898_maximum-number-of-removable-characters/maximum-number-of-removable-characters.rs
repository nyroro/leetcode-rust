
impl Solution {
    pub fn maximum_removals(s: String, p: String, removable: Vec<i32>) -> i32 {
        let mut s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();
        let mut left = 0;
        let mut right = removable.len() - 1;
        let mut result = 0;

        while left <= right {
            let mid = (left + right) / 2;
            let mut temp_chars = s_chars.clone();

            for i in 0..=mid {
                temp_chars[removable[i] as usize] = '*';
            }

            let temp_str: String = temp_chars.iter().collect();
            let is_subsequence = Self::is_subsequence(&temp_str, &p_chars);

            if is_subsequence {
                result = mid + 1;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        result

    }

    fn is_subsequence(s: &str, p: &[char]) -> bool {
        let mut p_index = 0;

        for c in s.chars() {
            if c == p[p_index] {
                p_index += 1;
            }

            if p_index == p.len() {
                return true;
            }
        }

        false

    }
}
