
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            let mut count = 0;
            for num in &nums {
                count += (num >> i) & 1;
            }
            result |= (count % 3) << i;
        }
        result

    }
}
