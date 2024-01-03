
impl Solution {
    pub fn can_be_increasing(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            let mut temp = nums.clone();
            temp.remove(i);
            if Self::is_increasing(&temp) {
                return true;
            }
        }
        false

    }

    fn is_increasing(nums: &Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if nums[i - 1] >= nums[i] {
                return false;
            }
        }
        true

    }
}
