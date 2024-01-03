
impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        // 定义旋转函数

        fn rotate(mat: &mut Vec<Vec<i32>>) {
            let n = mat.len();
            for i in 0..n / 2 {
                for j in i..n - i - 1 {
                    let temp = mat[i][j];
                    mat[i][j] = mat[n - j - 1][i];
                    mat[n - j - 1][i] = mat[n - i - 1][n - j - 1];
                    mat[n - i - 1][n - j - 1] = mat[j][n - i - 1];
                    mat[j][n - i - 1] = temp;
                }
            }
        }

        let mut mat = mat;
        for _ in 0..4 {
            rotate(&mut mat);
            if mat == target {
                return true;
            }
        }
        false

    }
}
