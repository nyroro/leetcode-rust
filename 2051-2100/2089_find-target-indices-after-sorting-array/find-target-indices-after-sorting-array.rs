
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices: Vec<i32> = Vec::new();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        for (i, num) in sorted_nums.iter().enumerate() {
            if *num == target {
                indices.push(i as i32);
            }
        }

        indices

    }
}
