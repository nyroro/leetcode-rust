
impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        // Initialize a counter to keep track of the number of matching items

        let mut counter = 0;
        
        // Determine the index based on the rule key

        let rule_key_index = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => 0,
        };
        
        // Iterate through the items and check for matches

        for item in items.iter() {
            if item[rule_key_index] == rule_value {
                counter += 1;
            }
        }
        
        // Return the count of matching items

        counter

    }
}
