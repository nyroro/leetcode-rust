
impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut result = String::new();  // Step 1: Create an empty string to store the result

        for i in (1..s.len()).step_by(2) {  // Step 2: Iterate through the odd indices of the string

            let c = s.chars().nth(i - 1).unwrap();  // Get the character at the previous index

            let x = s.chars().nth(i).unwrap().to_digit(10).unwrap() as u8 - b'0';  // Get the digit at the current index

            let shifted_char = (c as u8 + x) as char;  // Calculate the shifted character

            result.push(c);  // Append the original character to the result string

            result.push(shifted_char);  // Append the shifted character to the result string

        }
        if s.len() % 2 != 0 {  // If the length of the string is odd, there is an extra character at the end

            result.push(s.chars().last().unwrap());  // Append the last character to the result string

        }
        result  // Step 4: Return the result

    }
}
