
impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let mut i = 0;
        let mut j = 0;
        while i < groups.len() && j < nums.len() {
            let mut k = j;
            while k < nums.len() && groups[i] != &nums[k..k + groups[i].len()] {
                k += 1;
            }
            if k == nums.len() {
                return false;
            }
            i += 1;
            j = k + groups[i - 1].len();
        }
        i == groups.len()
    }
}
