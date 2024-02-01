
impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        // Convert the number to a string

        let num_str = num.to_string();
        // Convert the string to a character array

        let num_chars: Vec<char> = num_str.chars().collect();
        
        // Array to store the replacement digits for 0 to 9

        let replacements = [9, 1, 2, 3, 4, 5, 6, 7, 8, 0];
        
        // Initialize the minimum and maximum numbers with the original number

        let mut min_num = num;
        let mut max_num = num;
        
        // Iterate through each digit of the number

        for i in 0..num_chars.len() {
            // Convert the character to a digit

            let digit = num_chars[i].to_digit(10).unwrap() as usize;
            
            // Replace the digit with the corresponding replacement

            let replaced_min = num_str.replace(num_chars[i], replacements[digit].to_string().as_str());
            let replaced_max = num_str.replace(num_chars[i], replacements[digit].to_string().as_str());
            
            // Update the minimum and maximum numbers

            min_num = std::cmp::min(min_num, replaced_min.parse::<i32>().unwrap());
            max_num = std::cmp::max(max_num, replaced_max.parse::<i32>().unwrap());
        }
        
        // Calculate the difference between the maximum and minimum numbers

        let difference = max_num - min_num;
        
        // Return the difference

        difference

    }
}
