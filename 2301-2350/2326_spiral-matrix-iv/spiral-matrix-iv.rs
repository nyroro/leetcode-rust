
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        // 创建一个 m x n 的二维矩阵，初始值全部为 -1

        let mut matrix = vec![vec![-1; n as usize]; m as usize];
        
        // 定义四个方向：右、下、左、上

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut direction = 0;  // 当前方向的索引

        let (mut row, mut col) = (0, 0);  // 当前位置的行列索引
        
        // 遍历链表，将链表中的元素按照顺序填充到二维矩阵中

        let mut current = head;
        while let Some(node) = current {
            matrix[row as usize][col as usize] = node.val;
            current = node.next;
            
            // 计算下一个位置的行列索引

            let (next_row, next_col) = (row as i32 + directions[direction].0, col as i32 + directions[direction].1);
            
            // 如果下一个位置超出边界或者已经被填充过，则改变方向

            if next_row < 0 || next_row >= m || next_col < 0 || next_col >= n || matrix[next_row as usize][next_col as usize] != -1 {
                direction = (direction + 1) % 4;
            }
            
            // 更新当前位置的行列索引

            row = row + directions[direction].0;
            col = col + directions[direction].1;
        }
        
        // 返回生成的二维矩阵

        matrix

    }
}
