
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        let rows = grid.len();
        let cols = grid[0].len();
        
        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == 1 {
                    perimeter += 4; // 每个陆地的初始周长为4
                    
                    // 检查上方是否为水域或边界

                    if i > 0 && grid[i-1][j] == 1 {
                        perimeter -= 2; // 如果上方也是陆地，则减去2

                    }
                    
                    // 检查左方是否为水域或边界

                    if j > 0 && grid[i][j-1] == 1 {
                        perimeter -= 2; // 如果左方也是陆地，则减去2

                    }
                }
            }
        }
        
        perimeter

    }
}
