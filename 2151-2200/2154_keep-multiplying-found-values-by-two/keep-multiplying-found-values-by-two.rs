
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut result = original;
        let mut index = nums.iter().position(|&x| x == result);
        while let Some(i) = index {
            result *= 2;
            index = nums.iter().position(|&x| x == result);
        }
        result

    }
}
