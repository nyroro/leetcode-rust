
impl Solution {
    pub fn sum_imbalance_numbers(nums: Vec<i32>) -> i32 {
        let mut total_imbalance = 0;

        for start in 0..nums.len() {
            let mut sorted_subarray = Vec::new();
            for end in start..nums.len() {
                sorted_subarray.push(nums[end]);
                sorted_subarray.sort();
                let mut imbalance_count = 0;
                for i in 0..sorted_subarray.len() - 1 {
                    if sorted_subarray[i + 1] - sorted_subarray[i] > 1 {
                        imbalance_count += 1;
                    }
                }
                total_imbalance += imbalance_count;
            }
        }

        total_imbalance

    }
}
