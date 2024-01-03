
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        let mut sum = 0;
        let mut prefix_sum = std::collections::HashMap::new();
        prefix_sum.insert(0, 1);

        for num in nums {
            sum += num;
            if let Some(&prev_count) = prefix_sum.get(&(sum - k)) {
                count += prev_count;
            }
            *prefix_sum.entry(sum).or_insert(0) += 1;
        }

        count

    }
}
