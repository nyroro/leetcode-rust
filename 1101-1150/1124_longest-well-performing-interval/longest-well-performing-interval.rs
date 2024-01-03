


impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut max_length = 0;
        let mut prefix_sum_index = std::collections::HashMap::new();
        let mut prefix_sum = 0;
        
        for (index, &hour) in hours.iter().enumerate() {
            if hour > 8 {
                prefix_sum += 1;
            } else {
                prefix_sum -= 1;
            }
            
            if !prefix_sum_index.contains_key(&prefix_sum) {
                prefix_sum_index.insert(prefix_sum, index);
            }
            
            if prefix_sum > 0 {
                max_length = index as i32 + 1;
            } else {
                if let Some(&start_index) = prefix_sum_index.get(&(prefix_sum - 1)) {
                    max_length = max_length.max(index as i32 - start_index as i32);
                }
            }
        }
        
        max_length

    }
}
