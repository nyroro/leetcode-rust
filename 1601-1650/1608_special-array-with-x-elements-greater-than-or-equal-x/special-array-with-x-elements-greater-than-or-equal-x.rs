
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        for x in 1..=nums.len() {
            let count = nums.iter().filter(|&num| *num >= x as i32).count();
            if count == x {
                return x as i32;
            }
        }

        -1

    }
}
