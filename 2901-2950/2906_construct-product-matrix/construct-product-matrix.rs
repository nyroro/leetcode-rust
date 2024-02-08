


impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        
        let mut left = vec![vec![1; m]; n];
        let mut right = vec![vec![1; m]; n];
        
        let mut prev = 1;
        for i in 0..n {
            for j in 0..m {
                left[i][j] = prev;
                prev = (((prev % 12345) * (grid[i][j] % 12345)) % 12345);
            }
        }
        
        let mut last = 1;
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                right[i][j] = last;
                last = (((last % 12345) * (grid[i][j] % 12345)) % 12345);
            }
        }
        
        let mut result = vec![vec![1; m]; n];
        for i in 0..n {
            for j in 0..m {
                result[i][j] = (left[i][j] * right[i][j]) % 12345;
            }
        }
        
        result

    }
}
