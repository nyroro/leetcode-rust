
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        
        let m = matrix.len();
        let n = matrix[0].len();
        let mut row = 0;
        let mut col = n as i32 - 1; // Start from the rightmost column
        
        while row < m && col >= 0 {
            if matrix[row][col as usize] == target {
                return true;
            } else if matrix[row][col as usize] < target {
                row += 1; // Move down a row

            } else {
                col -= 1; // Move left a column

            }
        }
        
        false

    }
}
