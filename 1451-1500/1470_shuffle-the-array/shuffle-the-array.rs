
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..n {
            result.push(nums[i as usize]);
            result.push(nums[(i + n) as usize]);
        }
        result

    }
}
