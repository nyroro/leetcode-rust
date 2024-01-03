
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut rearranged_nums = vec![0; nums.len()];
        let mut even_index = 0;
        let mut odd_index = 1;

        for num in sorted_nums {
            if odd_index < nums.len() {
                rearranged_nums[odd_index] = num;
                odd_index += 2;
            } else {
                rearranged_nums[even_index] = num;
                even_index += 2;
            }
        }

        rearranged_nums

    }
}
