
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut xy = 0;
        let mut xz = 0;
        let mut yz = 0;
        
        for i in 0..n {
            let mut max_xz = 0;
            let mut max_yz = 0;
            for j in 0..n {
                if grid[i][j] > 0 {
                    xy += 1;
                }
                max_xz = max_xz.max(grid[i][j]);
                max_yz = max_yz.max(grid[j][i]);
            }
            xz += max_xz;
            yz += max_yz;
        }
        
        xy + xz + yz

    }
}
