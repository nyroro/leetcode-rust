
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        // Find the minimum element in the array

        let min_num = nums.iter().min().unwrap_or(&0);
        // Calculate the sum of the differences between each element and the minimum element

        let mut moves = 0;
        for &num in &nums {
            moves += num - min_num;
        }
        moves

    }
}
