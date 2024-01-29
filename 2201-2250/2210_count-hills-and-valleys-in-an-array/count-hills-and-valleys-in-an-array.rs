


impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut hill_valley = 0;
        let mut modified_nums = nums.clone();

        for i in 1..nums.len() - 1 {
            if modified_nums[i] == modified_nums[i + 1] {
                modified_nums[i] = modified_nums[i - 1];
            }
            if modified_nums[i] > modified_nums[i - 1] && modified_nums[i] > modified_nums[i + 1] {
                hill_valley += 1;
            }
            if modified_nums[i] < modified_nums[i - 1] && modified_nums[i] < modified_nums[i + 1] {
                hill_valley += 1;
            }
        }

        hill_valley

    }
}
