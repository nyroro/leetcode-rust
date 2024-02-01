
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left = 0;
        let mut right = m * n - 1;
        
        while left <= right {
            let mid = (left + right) / 2;
            let row = mid / n;
            let col = mid % n;
            
            if matrix[row][col] == target {
                return true;
            } else if matrix[row][col] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        false

    }
}
