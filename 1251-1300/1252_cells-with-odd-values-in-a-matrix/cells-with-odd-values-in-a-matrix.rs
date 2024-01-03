
impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        // 创建一个大小为m x n的矩阵，并将所有元素初始化为0

        let mut matrix = vec![vec![0; n as usize]; m as usize];
        
        // 对indices数组中的每个位置进行增量操作

        for index in indices {
            let row = index[0] as usize;
            let col = index[1] as usize;
            
            // 增加对应行的所有元素

            for i in 0..n as usize {
                matrix[row][i] += 1;
            }
            
            // 增加对应列的所有元素

            for i in 0..m as usize {
                matrix[i][col] += 1;
            }
        }
        
        // 统计奇数值的单元格数量

        let mut count = 0;
        for i in 0..m as usize {
            for j in 0..n as usize {
                if matrix[i][j] % 2 != 0 {
                    count += 1;
                }
            }
        }
        
        count

    }
}
