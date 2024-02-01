
use std::collections::HashMap;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut ratio_count: HashMap<f64, i64> = HashMap::new();
        let mut result: i64 = 0;

        for rectangle in rectangles {
            let ratio = rectangle[0] as f64 / rectangle[1] as f64;
            *ratio_count.entry(ratio).or_insert(0) += 1;
        }

        for (_, count) in ratio_count {
            if count > 1 {
                result += count * (count - 1) / 2;
            }
        }

        result

    }
}
