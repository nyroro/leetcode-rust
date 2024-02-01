
impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut result = vec![-1; n as usize];
        
        for i in 0..n {
            let mut row = 0;
            let mut col = i;
            while row < m {
                if grid[row][col] == 1 {
                    if col + 1 < n && grid[row][col + 1] == 1 {
                        row += 1;
                        col += 1;
                    } else {
                        break;
                    }
                } else {
                    if col > 0 && grid[row][col - 1] == -1 {
                        row += 1;
                        col -= 1;
                    } else {
                        break;
                    }
                }
            }
            if row == m {
                result[i as usize] = col;
            }
        }
        
        result

    }
}
