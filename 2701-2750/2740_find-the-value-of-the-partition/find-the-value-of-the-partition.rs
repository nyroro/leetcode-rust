
impl Solution {
    pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let n = sorted_nums.len();
        let mut min_partition_value = std::i32::MAX;

        for i in 0..n-1 {
            let diff1 = (sorted_nums[i] - sorted_nums[0]).abs();
            let diff2 = (sorted_nums[n-1] - sorted_nums[i+1]).abs();
            min_partition_value = min_partition_value.min(diff1).min(diff2);
        }

        min_partition_value

    }
}
