
impl Solution {
    pub fn smallest_beautiful_string(s: String, k: i32) -> String {
        let mut s = s.into_bytes(); // Convert the input string to bytes for easier manipulation

        let n = s.len();

        // Starting from the end of the string

        for i in (0..n).rev() {
            // Loop through all the possible characters that can be placed at the current position

            for ch in (s[i] + 1)..('a' as u8 + k as u8) {
                // Check if the current character can be placed at this position

                if (i == 0 || s[i - 1] != ch) && (i <= 1 || s[i - 2] != ch) {
                    // If the current character can be placed, update the string and fill in the remaining positions

                    s[i] = ch;
                    for j in (i + 1)..n {
                        // Loop through all the possible characters that can be placed at the remaining positions

                        for cand in 'a' as u8..('a' as u8 + k as u8) {
                            // Check if the current character can be placed at this position

                            if cand != s[j - 1] && (j == 1 || cand != s[j - 2]) {
                                // If the current character can be placed, update the string and break from the loop

                                s[j] = cand;
                                break;
                            }
                        }
                    }
                    // Return the updated string

                    return String::from_utf8(s).unwrap();
                }
            }
        }
        // If no such string is possible, return an empty string

        String::from("")
    }
}
