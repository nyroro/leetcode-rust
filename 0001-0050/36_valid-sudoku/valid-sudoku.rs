
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashSet::new(); 9];
        let mut cols = vec![HashSet::new(); 9];
        let mut boxes = vec![HashSet::new(); 9];
        
        for i in 0..9 {
            for j in 0..9 {
                let num = board[i][j];
                if num != '.' {
                    let num = num.to_digit(10).unwrap() as usize;
                    let box_index = (i / 3) * 3 + j / 3;
                    
                    if rows[i].contains(&num) || cols[j].contains(&num) || boxes[box_index].contains(&num) {
                        return false;
                    }
                    
                    rows[i].insert(num);
                    cols[j].insert(num);
                    boxes[box_index].insert(num);
                }
            }
        }
        
        true

    }
}
