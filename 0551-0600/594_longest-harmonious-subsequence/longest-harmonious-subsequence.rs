
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut count: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }

        let mut max_length = 0;
        for (&num, &freq) in count.iter() {
            if count.contains_key(&(num + 1)) {
                max_length = max_length.max(freq + count[&(num + 1)]);
            }
        }

        max_length

    }
}
