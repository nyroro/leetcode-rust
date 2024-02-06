
use std::collections::HashMap;

impl Solution {
    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        let mut data: HashMap<i32, i32> = HashMap::new();

        // Count the frequency of each unique integer in nums

        for num in nums {
            *data.entry(num).or_insert(0) += 1;
        }

        // Sort the quantity vector in descending order

        let mut sorted_quantity = quantity.clone();
        sorted_quantity.sort_unstable_by(|a, b| b.cmp(a));

        // Define a recursive function to check if it's possible to distribute the integers

        fn recursive(data: &HashMap<i32, i32>, quantity: &Vec<i32>, j: usize) -> bool {
            if j == quantity.len() {
                return true;
            }
            let elem = quantity[j];

            for (&num, &freq) in data.iter() {
                if freq >= elem {
                    let mut data_copy = data.clone();
                    *data_copy.entry(num).or_insert(0) -= elem;
                    if recursive(&data_copy, quantity, j + 1) {
                        return true;
                    }
                }
            }
            false

        }

        // Call the recursive function with an initial index of 0

        recursive(&data, &sorted_quantity, 0)
    }
}
