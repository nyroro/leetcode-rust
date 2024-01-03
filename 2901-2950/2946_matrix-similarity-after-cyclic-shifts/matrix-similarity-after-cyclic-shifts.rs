
impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let m = mat.len();
        let n = mat[0].len();
        let mut new_mat = mat.clone();

        for i in 0..m {
            if i % 2 == 1 {
                // 循环右移奇数行k次

                for _ in 0..k {
                    let last = new_mat[i][n - 1];
                    for j in (1..n).rev() {
                        new_mat[i][j] = new_mat[i][j - 1];
                    }
                    new_mat[i][0] = last;
                }
            } else {
                // 循环左移偶数行k次

                for _ in 0..k {
                    let first = new_mat[i][0];
                    for j in 0..n - 1 {
                        new_mat[i][j] = new_mat[i][j + 1];
                    }
                    new_mat[i][n - 1] = first;
                }
            }
        }

        mat == new_mat

    }
}
