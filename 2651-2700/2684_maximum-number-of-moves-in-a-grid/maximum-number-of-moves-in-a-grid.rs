
impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut max_moves = vec![0; m];

        for col in (0..n).rev() {
            let mut new_max_moves = vec![0; m];
            for row in 0..m {
                for dr in [-1, 0, 1].iter() {
                    let new_row = (row as i32 + dr) as usize;
                    if new_row < m && col + 1 < n && grid[new_row][col + 1] > grid[row][col] {
                        new_max_moves[row] = new_max_moves[row].max(max_moves[new_row] + 1);
                    }
                }
            }
            max_moves = new_max_moves;
        }

        *max_moves.iter().max().unwrap()
    }
}
