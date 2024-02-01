
impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < groups.len() && j < nums.len() {
            if let Some(k) = nums[j..].windows(groups[i].len()).position(|window| window == &groups[i][..]) {
                i += 1;
                j += k + groups[i - 1].len();
            } else {
                return false;
            }
        }
        i == groups.len()
    }
}
