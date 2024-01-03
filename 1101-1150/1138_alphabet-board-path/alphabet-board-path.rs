
use std::collections::HashMap;



impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let board = vec![
            "abcde", "fghij", "klmno", "pqrst", "uvwxy", "z"
        ];
        let mut char_to_pos: HashMap<char, (usize, usize)> = HashMap::new();
        
        for (r, row) in board.iter().enumerate() {
            for (c, ch) in row.chars().enumerate() {
                char_to_pos.insert(ch, (r, c));
            }
        }
        
        let mut result = String::new();
        let mut current_pos = (0, 0);
        
        for ch in target.chars() {
            let target_pos = char_to_pos[&ch];
            let (mut tr, mut tc) = target_pos;
            let (mut cr, mut cc) = current_pos;
            let mut moves = String::new();
            
            if ch == 'z' {
                while cr > tr {
                    moves.push('U');
                    cr -= 1;
                }
                while cc < tc {
                    moves.push('R');
                    cc += 1;
                }
                while cc > tc {
                    moves.push('L');
                    cc -= 1;
                }
                while cr < tr {
                    moves.push('D');
                    cr += 1;
                }
            } else {
                while cr > tr {
                    moves.push('U');
                    cr -= 1;
                }
                while cc < tc {
                    moves.push('R');
                    cc += 1;
                }
                while cc > tc {
                    moves.push('L');
                    cc -= 1;
                }
                while cr < tr {
                    moves.push('D');
                    cr += 1;
                }
            }
            
            moves.push('!');
            result.push_str(&moves);
            current_pos = target_pos;
        }
        
        result

    }
}
