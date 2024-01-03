
impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut left = vec![vec![0; n]; m];
        let mut top = vec![vec![0; n]; m];
        let mut res = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    left[i][j] = if j > 0 { left[i][j - 1] + 1 } else { 1 };
                    top[i][j] = if i > 0 { top[i - 1][j] + 1 } else { 1 };
                }
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                let mut len = std::cmp::min(left[i][j], top[i][j]);
                while len > res {
                    if top[i][j - len + 1] >= len && left[i - len + 1][j] >= len {
                        res = len;
                        break;
                    }
                    len -= 1;
                }
            }
        }

        (res * res) as i32

    }
}
