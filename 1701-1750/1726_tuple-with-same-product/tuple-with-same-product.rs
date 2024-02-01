
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                for k in 0..nums.len() {
                    for l in (k + 1)..nums.len() {
                        if nums[i] * nums[j] == nums[k] * nums[l] && nums[i] != nums[j] && nums[i] != nums[k] && nums[i] != nums[l] && nums[j] != nums[k] && nums[j] != nums[l] && nums[k] != nums[l] {
                            count += 8;
                        }
                    }
                }
            }
        }
        count

    }
}
