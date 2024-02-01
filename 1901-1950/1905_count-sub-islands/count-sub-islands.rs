
impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, grid2: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        let m = grid1.len();
        let n = grid1[0].len();
        
        fn dfs(grid1: &Vec<Vec<i32>>, grid2: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
            if i >= grid2.len() || j >= grid2[0].len() || grid2[i][j] == 0 {
                return true;
            }
            
            if grid1[i][j] == 0 {
                return false;
            }
            
            grid2[i][j] = 0;
            
            let mut is_sub_island = true;
            
            if i > 0 {
                is_sub_island &= dfs(grid1, grid2, i - 1, j);
            }
            
            if i < grid2.len() - 1 {
                is_sub_island &= dfs(grid1, grid2, i + 1, j);
            }
            
            if j > 0 {
                is_sub_island &= dfs(grid1, grid2, i, j - 1);
            }
            
            if j < grid2[0].len() - 1 {
                is_sub_island &= dfs(grid1, grid2, i, j + 1);
            }
            
            is_sub_island

        }
        
        for i in 0..m {
            for j in 0..n {
                if grid2[i][j] == 1 && dfs(&grid1, &grid2, i, j) {
                    count += 1;
                }
            }
        }
        
        count

    }
}
