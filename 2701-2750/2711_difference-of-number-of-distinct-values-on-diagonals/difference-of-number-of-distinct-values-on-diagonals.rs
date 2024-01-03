
impl Solution {
    pub fn difference_of_distinct_values(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Create a new 2D array to store the result

        let m = grid.len();
        let n = grid[0].len();
        let mut answer = vec![vec![0; n]; m];

        // Iterate through each cell of the grid

        for r in 0..m {
            for c in 0..n {
                // Calculate the distinct values in the top-left diagonal

                let mut top_left = std::collections::HashSet::new();
                let mut i = r;
                let mut j = c;
                while i > 0 && j > 0 {
                    i -= 1;
                    j -= 1;
                    top_left.insert(grid[i][j]);
                }

                // Calculate the distinct values in the bottom-right diagonal

                let mut bottom_right = std::collections::HashSet::new();
                let mut i = r;
                let mut j = c;
                while i < m - 1 && j < n - 1 {
                    i += 1;
                    j += 1;
                    bottom_right.insert(grid[i][j]);
                }

                // Calculate the absolute difference and store it in the answer array

                answer[r][c] = (top_left.len() as i32 - bottom_right.len() as i32).abs();
            }
        }

        // Return the result

        answer

    }
}
