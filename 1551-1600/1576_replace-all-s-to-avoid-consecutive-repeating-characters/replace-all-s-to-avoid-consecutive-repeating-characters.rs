
impl Solution {
    pub fn modify_string(s: String) -> String {
        // Convert the string to a mutable character array

        let mut chars: Vec<char> = s.chars().collect();
        
        // Iterate through the character array

        for i in 0..chars.len() {
            if chars[i] == '?' {
                // Replace '?' with a lowercase letter that is different from its adjacent characters

                let prev = if i == 0 { None } else { Some(chars[i - 1]) };
                let next = if i == chars.len() - 1 { None } else { Some(chars[i + 1]) };
                chars[i] = Self::get_non_repeating_char(prev, next);
            }
        }
        
        // Convert the character array back to a string and return

        chars.iter().collect()
    }
    
    // Function to get a lowercase letter that is different from its adjacent characters

    fn get_non_repeating_char(prev: Option<char>, next: Option<char>) -> char {
        for c in b'a'..=b'z' {
            let c = c as char;
            if Some(c) != prev && Some(c) != next {
                return c;
            }
        }
        'a' // Fallback to 'a' if no valid character is found

    }
}
