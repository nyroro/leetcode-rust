
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        if k < 0 {
            return 0;
        }

        let mut count = 0;
        let mut num_count = HashMap::new();

        for num in nums {
            *num_count.entry(num).or_insert(0) += 1;
        }

        for (num, freq) in num_count.iter() {
            if k == 0 {
                if *freq > 1 {
                    count += 1;
                }
            } else {
                if num_count.contains_key(&(num + k)) {
                    count += 1;
                }
            }
        }

        count

    }
}
