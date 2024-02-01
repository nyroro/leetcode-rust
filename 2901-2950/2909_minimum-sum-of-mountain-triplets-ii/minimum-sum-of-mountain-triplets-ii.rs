
impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let mut min_sum = -1;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] < nums[j] {
                    for k in (j + 1)..nums.len() {
                        if nums[j] > nums[k] {
                            let current_sum = nums[i] + nums[j] + nums[k];
                            if min_sum == -1 || current_sum < min_sum {
                                min_sum = current_sum;
                            }
                        }
                    }
                }
            }
        }
        min_sum

    }
}
