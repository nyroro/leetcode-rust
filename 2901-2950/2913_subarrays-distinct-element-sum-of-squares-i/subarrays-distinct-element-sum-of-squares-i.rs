
impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            let mut distinct_counts = std::collections::HashSet::new();
            for j in i..nums.len() {
                distinct_counts.insert(nums[j]);
                result += (distinct_counts.len() * distinct_counts.len()) as i32;
            }
        }
        result

    }
}
