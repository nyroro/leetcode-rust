
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;
        let m = grid.len();
        let n = grid[0].len();
        
        fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
            if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 {
                return 0;
            }
            
            grid[i][j] = 0;
            
            let mut area = 1;
            
            if i > 0 {
                area += dfs(grid, i - 1, j);
            }
            if i < grid.len() - 1 {
                area += dfs(grid, i + 1, j);
            }
            if j > 0 {
                area += dfs(grid, i, j - 1);
            }
            if j < grid[0].len() - 1 {
                area += dfs(grid, i, j + 1);
            }
            
            area

        }
        
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    let area = dfs(&mut grid.clone(), i, j);
                    max_area = max_area.max(area);
                }
            }
        }
        
        max_area

    }
}
