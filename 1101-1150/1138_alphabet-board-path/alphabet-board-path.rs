
// 定义一个结构体表示字母表板

struct AlphabetBoard {
    board: Vec<String>,
    current_position: (usize, usize),
}

impl AlphabetBoard {
    // 初始化字母表板

    fn new() -> AlphabetBoard {
        let board = vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "uvwxy".to_string(),
            "z".to_string(),
        ];
        AlphabetBoard {
            board,
            current_position: (0, 0),
        }
    }

    // 计算从当前位置到目标字符的移动路径

    fn calculate_path(&mut self, target_char: char) -> String {
        let target_position = self.find_position(target_char);
        let mut path = String::new();
        let (mut r, mut c) = self.current_position;

        while (r, c) != target_position {
            if r > target_position.0 {
                r -= 1;
                path.push('U');
            } else if r < target_position.0 {
                r += 1;
                path.push('D');
            }
            if c > target_position.1 {
                c -= 1;
                path.push('L');
            } else if c < target_position.1 {
                c += 1;
                path.push('R');
            }
        }

        self.current_position = target_position;
        path.push('!');
        path

    }

    // 查找字符在字母表板中的位置

    fn find_position(&self, target_char: char) -> (usize, usize) {
        for (r, row) in self.board.iter().enumerate() {
            if let Some(c) = row.find(target_char) {
                return (r, c);
            }
        }
        (0, 0)
    }
}

// 实现Solution结构体



impl Solution {
    pub fn alphabet_board_path(target: String) -> String {
        let mut board = AlphabetBoard::new();
        let mut path = String::new();

        for c in target.chars() {
            path += &board.calculate_path(c);
        }

        path

    }
}

fn main() {
    let target1 = "leet".to_string();
    let target2 = "code".to_string();

    println!("{}", Solution::alphabet_board_path(target1));
    println!("{}", Solution::alphabet_board_path(target2));
}
