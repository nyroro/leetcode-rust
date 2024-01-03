
use std::collections::HashSet;

impl Solution {
    pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
        let n = matrix.len();
        
        // 检查每一行

        for i in 0..n {
            let mut row_set: HashSet<i32> = HashSet::new();
            for j in 0..n {
                if !row_set.insert(matrix[i][j]) {
                    return false;
                }
            }
            if row_set.len() != n {
                return false;
            }
        }
        
        // 检查每一列

        for j in 0..n {
            let mut col_set: HashSet<i32> = HashSet::new();
            for i in 0..n {
                if !col_set.insert(matrix[i][j]) {
                    return false;
                }
            }
            if col_set.len() != n {
                return false;
            }
        }
        
        true

    }
}
