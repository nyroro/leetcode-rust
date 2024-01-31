
impl Solution {
    pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
        let n = nums.len();
        let mut ar = vec![vec![0; n]; n];
        
        // Populate the 2D vector ar

        for i in 0..n {
            for j in 0..n {
                ar[i][j] = nums[(j + i) % n];
            }
        }
        
        let mut ans = std::i64::MAX;
        let mut min_scores = vec![std::i64::MAX; n];
        
        // Iterate through the rotations

        for i in 0..n {
            for j in 0..n {
                min_scores[j] = min_scores[j].min(ar[i][j] as i64);
            }
            ans = ans.min(min_scores.iter().sum::<i64>() + (i as i64 * x as i64));
        }
        
        ans

    }
}
