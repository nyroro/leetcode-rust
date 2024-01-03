
impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut even_moves = 0;
        let mut odd_moves = 0;

        for i in 0..nums.len() {
            let left = if i > 0 { nums[i - 1] } else { std::i32::MAX };
            let right = if i < nums.len() - 1 { nums[i + 1] } else { std::i32::MAX };

            let target = std::cmp::min(left, right) - 1;

            if i % 2 == 0 {
                even_moves += std::cmp::max(0, nums[i] - target);
            } else {
                odd_moves += std::cmp::max(0, nums[i] - target);
            }
        }

        std::cmp::min(even_moves, odd_moves)
    }
}
