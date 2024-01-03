
impl Solution {
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let mut nums: Vec<String> = nums.into_iter().collect();
        nums.sort_unstable_by(|a, b| {
            if a.len() == b.len() {
                a.cmp(b)
            } else {
                a.len().cmp(&b.len())
            }
        });
        nums[nums.len() - k as usize].clone()
    }
}
