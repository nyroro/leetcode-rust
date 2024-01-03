
impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_hourglass_sum = i32::MIN;

        for i in 1..m - 1 {
            for j in 1..n - 1 {
                let current_hourglass_sum = grid[i][j] + grid[i-1][j-1] + grid[i-1][j] + grid[i-1][j+1] + grid[i+1][j-1] + grid[i+1][j] + grid[i+1][j+1];
                max_hourglass_sum = max_hourglass_sum.max(current_hourglass_sum);
            }
        }

        max_hourglass_sum

    }
}
