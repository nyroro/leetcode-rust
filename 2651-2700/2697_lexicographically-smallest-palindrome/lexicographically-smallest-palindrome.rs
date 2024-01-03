
impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        let (mut left, mut right) = (0, chars.len() - 1);
        let mut replace_count = 0;

        while left < right {
            if chars[left] != chars[right] {
                if chars[left] > chars[right] {
                    chars[left] = chars[right];
                } else {
                    chars[right] = chars[left];
                }
                replace_count += 1;
            }
            left += 1;
            right -= 1;
        }

        chars.iter().collect()
    }
}
