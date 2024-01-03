
impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();

        // 行翻转，确保每一行的第一个元素都为1

        for i in 0..m {
            if grid[i][0] == 0 {
                for j in 0..n {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }

        // 列翻转，使得每一列中0的个数大于1的个数

        for j in 1..n {
            let mut count = 0;
            for i in 0..m {
                if grid[i][j] == 0 {
                    count += 1;
                }
            }
            if count > m / 2 {
                for i in 0..m {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }

        // 计算得分

        let mut score = 0;
        for i in 0..m {
            let mut row_score = 0;
            for j in 0..n {
                row_score = row_score * 2 + grid[i][j];
            }
            score += row_score;
        }

        score

    }
}
