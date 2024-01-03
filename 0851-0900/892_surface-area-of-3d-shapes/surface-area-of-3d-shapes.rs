
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut area = 0;

        for i in 0..n {
            for j in 0..n {
                let height = grid[i][j];

                if height > 0 {
                    // 计算当前立方体的表面积

                    area += 2 + 4 * height;

                    // 减去与相邻立方体贴合的表面积

                    if i > 0 {
                        area -= 2 * std::cmp::min(height, grid[i - 1][j]);
                    }
                    if j > 0 {
                        area -= 2 * std::cmp::min(height, grid[i][j - 1]);
                    }
                }
            }
        }

        area

    }
}
