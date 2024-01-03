
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        for word in words {
            let mut left = 0;
            let mut right = word.len() - 1;
            let chars: Vec<char> = word.chars().collect();

            while left < right {
                if chars[left] != chars[right] {
                    break;
                }
                left += 1;
                right -= 1;
            }

            if left >= right {
                return word;
            }
        }

        String::from("")
    }
}
