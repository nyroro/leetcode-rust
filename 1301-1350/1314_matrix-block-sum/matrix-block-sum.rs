
impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let mut answer = vec![vec![0; n]; m];
        
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0;
                for r in (i as i32 - k).max(0)..=(i as i32 + k).min(m as i32 - 1) {
                    for c in (j as i32 - k).max(0)..=(j as i32 + k).min(n as i32 - 1) {
                        sum += mat[r as usize][c as usize];
                    }
                }
                answer[i][j] = sum;
            }
        }
        
        answer

    }
}
