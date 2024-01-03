
impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 初始化矩阵

        let mut mat = vec![vec![0; n as usize]; n as usize];
        
        // 遍历查询

        for query in queries {
            let row1 = query[0] as usize;
            let col1 = query[1] as usize;
            let row2 = query[2] as usize;
            let col2 = query[3] as usize;
            
            // 更新子矩阵中的每个元素

            for i in row1..=row2 {
                for j in col1..=col2 {
                    mat[i][j] += 1;
                }
            }
        }
        
        mat

    }
}
