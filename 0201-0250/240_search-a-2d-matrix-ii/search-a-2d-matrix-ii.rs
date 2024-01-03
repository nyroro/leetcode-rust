
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut i = 0;
        let mut j = matrix[0].len() as i32 - 1;
        while i < matrix.len() as i32 && j >= 0 {
            if matrix[i as usize][j as usize] == target {
                return true;
            } else if matrix[i as usize][j as usize] > target {
                j -= 1;
            } else {
                i += 1;
            }
        }
        false

    }
}
