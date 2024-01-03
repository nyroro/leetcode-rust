
impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        // 创建一个 8x8 的二维数组表示棋盘

        let mut board = [[0; 8]; 8];
        
        // 标记黑皇后的位置

        for queen in &queens {
            board[queen[0] as usize][queen[1] as usize] = 1;
        }
        
        // 存储可以直接攻击白王的黑皇后的坐标

        let mut result = Vec::new();
        
        // 检查每个黑皇后是否可以直接攻击白王

        let directions = vec![-1, 0, 1];
        for i in &directions {
            for j in &directions {
                if *i == 0 && *j == 0 {
                    continue;
                }
                let mut x = king[0];
                let mut y = king[1];
                while x >= 0 && x < 8 && y >= 0 && y < 8 {
                    if board[x as usize][y as usize] == 1 {
                        result.push(vec![x, y]);
                        break;
                    }
                    x += i;
                    y += j;
                }
            }
        }
        
        result

    }
}
