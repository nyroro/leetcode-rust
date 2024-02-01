
impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        // Create an empty string to store the modified sentence

        let mut modified_sentence = String::new();
        
        // Split the input sentence into words

        let words: Vec<&str> = sentence.split_whitespace().collect();
        
        // Iterate through each word in the sentence

        for word in words {
            // Check if the word represents a price

            if word.starts_with('$') {
                // Extract the numeric part of the price

                let price = &word[1..];
                
                // Parse the price as a floating-point number

                if let Ok(mut price_value) = price.parse::<f64>() {
                    // Apply the discount to the price

                    price_value *= 1.0 - (discount as f64 / 100.0);
                    
                    // Format the price with exactly two decimal places

                    let formatted_price = format!("{:.2}", price_value);
                    
                    // Update the modified sentence with the formatted price

                    modified_sentence.push_str(&format!("${} ", formatted_price));
                } else {
                    // If parsing the price fails, add the original word to the modified sentence

                    modified_sentence.push_str(&format!("{} ", word));
                }
            } else {
                // If the word does not represent a price, add it to the modified sentence

                modified_sentence.push_str(&format!("{} ", word));
            }
        }
        
        // Return the modified sentence after trimming any trailing whitespace

        modified_sentence.trim().to_string()
    }
}
