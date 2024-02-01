
use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut count_map: HashMap<String, i32> = HashMap::new();
        let mut max_count = 0;

        for row in matrix {
            let mut row_str = String::new();
            let mut flip_row_str = String::new();

            for num in row {
                row_str.push_str(&num.to_string());
                flip_row_str.push_str(&(1 - num).to_string());
            }

            let count = count_map.entry(row_str.clone()).or_insert(0);
            *count += 1;

            let flip_count = count_map.entry(flip_row_str.clone()).or_insert(0);
            *flip_count += 1;

            max_count = max_count.max(*count).max(*flip_count);
        }

        max_count

    }
}
