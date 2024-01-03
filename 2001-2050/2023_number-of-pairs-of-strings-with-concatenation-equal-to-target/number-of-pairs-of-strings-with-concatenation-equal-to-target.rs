
impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let mut count = 0;
        let n = nums.len();
        
        for i in 0..n {
            for j in 0..n {
                if i != j && nums[i].to_owned() + &nums[j] == target {
                    count += 1;
                }
            }
        }
        
        count

    }
}
