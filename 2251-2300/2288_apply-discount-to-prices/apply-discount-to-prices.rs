
impl Solution {
    pub fn discount_prices(sentence: String, discount: i32) -> String {
        let mut modified_sentence = String::new();
        let words: Vec<&str> = sentence.split_whitespace().collect();
        
        for word in words {
            if word.starts_with('$') {
                let price = &word[1..];
                
                // Check if the price is a valid number

                if let Ok(price_value) = price.parse::<f64>() {
                    // Check if the price is a valid number and does not contain 'e'
                    if !price.contains('e') {
                        // Calculate the discounted price

                        let discounted_price = price_value * (1.0 - (discount as f64 / 100.0));
                        
                        // Format the price with exactly two decimal places

                        let formatted_price = format!("{:.2}", discounted_price);
                        
                        // Update the modified sentence with the formatted price

                        modified_sentence.push_str(&format!("${} ", formatted_price));
                        continue;
                    }
                }
            }
            
            // If the word does not represent a price or is not a valid price, add it to the modified sentence

            modified_sentence.push_str(&format!("{} ", word));
        }
        
        modified_sentence.trim().to_string()
    }
}
