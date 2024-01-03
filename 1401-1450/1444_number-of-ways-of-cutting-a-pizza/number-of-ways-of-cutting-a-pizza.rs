
impl Solution {
    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let rows = pizza.len();
        let cols = pizza[0].len();
        let mut apples = vec![vec![0; cols + 1]; rows + 1];
        for i in (0..rows).rev() {
            let mut row_sum = 0;
            for j in (0..cols).rev() {
                if &pizza[i][j..j + 1] == "A" {
                    row_sum += 1;
                }
                apples[i][j] = apples[i + 1][j] + row_sum;
            }
        }
        
        let mut dp = vec![vec![vec![-1; k as usize]; cols as usize]; rows as usize];
        Solution::cut_ways(0, 0, k as usize - 1, &apples, &mut dp) % 1_000_000_007

    }
    
    fn cut_ways(i: usize, j: usize, s: usize, apples: &Vec<Vec<i32>>, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if apples[i][j] == 0 {
            return 0;
        }
        if s == 0 {
            return 1;
        }
        if dp[i][j][s] != -1 {
            return dp[i][j][s];
        }
        
        let mut ways = 0;
        for x in i + 1..apples.len() {
            if apples[i][j] - apples[x][j] > 0 {
                ways = (ways + Solution::cut_ways(x, j, s - 1, apples, dp)) % 1_000_000_007;
            }
        }
        for y in j + 1..apples[0].len() {
            if apples[i][j] - apples[i][y] > 0 {
                ways = (ways + Solution::cut_ways(i, y, s - 1, apples, dp)) % 1_000_000_007;
            }
        }
        dp[i][j][s] = ways;
        ways

    }
}
