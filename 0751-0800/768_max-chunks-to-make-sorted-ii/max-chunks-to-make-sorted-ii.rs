
use std::iter::FromIterator;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut acc_original = 0;
        let mut acc_sorted = 0;
        let mut count = 0;

        let mut sorted_arr: Vec<i32> = arr.iter().cloned().collect();
        sorted_arr.sort();

        for (original, sorted) in arr.iter().zip(sorted_arr.iter()) {
            acc_original += original;
            acc_sorted += sorted;

            if acc_original == acc_sorted {
                count += 1;
            }
        }

        count

    }
}
