
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut longest = 0;
        let mut current = 0;
        let mut maximum = 0;

        for &num in nums.iter() {
            if num > maximum {
                maximum = num;
                longest = 0;
                current = 0;
            }
            if num == maximum {
                current += 1;
            } else {
                current = 0;
            }
            longest = longest.max(current);
        }

        longest

    }
}
