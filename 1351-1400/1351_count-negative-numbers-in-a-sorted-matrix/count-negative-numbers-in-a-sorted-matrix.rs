
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let mut row = 0;
        let mut col = grid[0].len() as i32 - 1;
        
        while (row as usize) < grid.len() && col >= 0 {
            if grid[row as usize][col as usize] < 0 {
                count += grid.len() as i32 - row;
                col -= 1;
            } else {
                row += 1;
            }
        }
        
        count

    }
}
