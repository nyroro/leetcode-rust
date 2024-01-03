
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut row_count = vec![0; m];
        let mut col_count = vec![0; n];
        let mut special_count = 0;

        // 计算每行和每列中元素为 1 的数量

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    row_count[i] += 1;
                    col_count[j] += 1;
                }
            }
        }

        // 遍历整个矩阵，统计特殊位置的数量

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 && row_count[i] == 1 && col_count[j] == 1 {
                    special_count += 1;
                }
            }
        }

        special_count

    }
}
