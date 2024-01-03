
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut subsequence_count = 0;
        let mut current_subsequence = vec![];

        for num in nums {
            if current_subsequence.is_empty() {
                current_subsequence.push(num);
            } else {
                let diff = num - *current_subsequence.first().unwrap();
                if diff > k {
                    subsequence_count += 1;
                    current_subsequence.clear();
                }
                current_subsequence.push(num);
            }
        }

        if !current_subsequence.is_empty() {
            subsequence_count += 1;
        }

        subsequence_count

    }
}
