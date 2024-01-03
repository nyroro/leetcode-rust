
use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut product_count: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let product = nums[i] * nums[j];
                count += *product_count.get(&product).unwrap_or(&0);
                *product_count.entry(product).or_insert(0) += 1;
            }
        }

        count * 8

    }
}
