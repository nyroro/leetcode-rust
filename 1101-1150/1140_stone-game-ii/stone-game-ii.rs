
impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut sums = piles.clone();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        
        for i in (0..n-1).rev() {
            sums[i] += sums[i + 1];
        }
        
        Solution::helper(&sums, 0, 1, &mut dp)
    }
    
    fn helper(sums: &Vec<i32>, i: usize, m: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        let n = sums.len();
        if i + 2 * m >= n {
            return sums[i];
        }
        if memo[i][m] > 0 {
            return memo[i][m];
        }
        let mut res = 0;
        for x in 1..=2 * m {
            if i + x <= n {
                let cur = sums[i] - sums[i + x];
                res = res.max(cur + sums[i + x] - Solution::helper(sums, i + x, x.max(m), memo));
            }
        }
        memo[i][m] = res;
        res

    }
}
