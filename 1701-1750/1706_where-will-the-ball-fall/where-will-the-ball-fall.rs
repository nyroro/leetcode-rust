
impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut result = vec![-1; n as usize];
        
        for i in 0..n {
            let mut row = 0;
            let mut col = i;
            while row < m {
                if grid[row as usize][col as usize] == 1 {
                    if col + 1 < n && grid[row as usize][(col + 1) as usize] == 1 {
                        row += 1;
                        col += 1;
                    } else {
                        break;
                    }
                } else {
                    if col > 0 && grid[row as usize][(col - 1) as usize] == -1 {
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
