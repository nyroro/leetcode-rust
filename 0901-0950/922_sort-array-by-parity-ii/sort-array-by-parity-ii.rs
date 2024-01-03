
impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut even_index = 0;
        let mut odd_index = 1;
        let len = nums.len();

        while even_index < len && odd_index < len {
            while even_index < len && nums[even_index] % 2 == 0 {
                even_index += 2;
            }

            while odd_index < len && nums[odd_index] % 2 == 1 {
                odd_index += 2;
            }

            if even_index < len && odd_index < len {
                nums.swap(even_index, odd_index);
            }
        }

        nums

    }
}
