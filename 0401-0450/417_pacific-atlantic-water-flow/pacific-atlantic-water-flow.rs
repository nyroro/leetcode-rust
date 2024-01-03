
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut pacific = vec![vec![false; n]; m];
        let mut atlantic = vec![vec![false; n]; m];
        let mut result = Vec::new();

        // DFS for cells that can flow to the Pacific

        for i in 0..m {
            Solution::dfs(&heights, &mut pacific, i, 0);
        }
        for j in 0..n {
            Solution::dfs(&heights, &mut pacific, 0, j);
        }

        // DFS for cells that can flow to the Atlantic

        for i in 0..m {
            Solution::dfs(&heights, &mut atlantic, i, n - 1);
        }
        for j in 0..n {
            Solution::dfs(&heights, &mut atlantic, m - 1, j);
        }

        // Find cells that can flow to both the Pacific and Atlantic

        for i in 0..m {
            for j in 0..n {
                if pacific[i][j] && atlantic[i][j] {
                    result.push(vec![i as i32, j as i32]);
                }
            }
        }

        result

    }

    fn dfs(heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, row: usize, col: usize) {
        visited[row][col] = true;
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        let m = heights.len();
        let n = heights[0].len();

        for (dx, dy) in directions {
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;

            if new_row >= 0 && new_row < m as i32 && new_col >= 0 && new_col < n as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                if !visited[new_row][new_col] && heights[new_row][new_col] >= heights[row][col] {
                    Solution::dfs(heights, visited, new_row, new_col);
                }
            }
        }
    }
}
