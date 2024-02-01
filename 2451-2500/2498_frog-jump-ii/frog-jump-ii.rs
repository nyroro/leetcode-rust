
impl Solution {
    pub fn max_jump(stones: Vec<i32>) -> i32 {
        let n = stones.len();
        let mut max_jumps = vec![0; n];
        max_jumps[1] = stones[1];  // Set the initial value for max_jumps[1]

        for i in 2..n {
            max_jumps[i] = std::cmp::max(max_jumps[i - 1], stones[i] - stones[i - 2]);
        }
        
        max_jumps[n - 1]  // Return the last element of max_jumps

    }
}
