
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max_length = -1;
        let mut char_indices = vec![-1; 26]; // Initialize with -1 for each lowercase English letter


        for (i, c) in s.chars().enumerate() {
            let index = (c as u8 - b'a') as usize;
            if char_indices[index] == -1 {
                char_indices[index] = i as i32; // Record the first occurrence of the character

            } else {
                max_length = max_length.max((i as i32 - char_indices[index] - 1)); // Calculate the length of the substring

            }
        }

        max_length

    }
}
