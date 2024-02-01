
// 定义 SubrectangleQueries 结构体

struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
    history: Vec<(i32, i32, i32, i32, i32)>,
}

impl SubrectangleQueries {
    // 实现 new 方法

    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        SubrectangleQueries {
            rectangle,
            history: Vec::new(),
        }
    }

    // 实现 update_subrectangle 方法

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        self.history.push((row1, col1, row2, col2, new_value));
    }

    // 实现 get_value 方法

    fn get_value(&self, row: i32, col: i32) -> i32 {
        for i in (0..self.history.len()).rev() {
            let (row1, col1, row2, col2, val) = self.history[i];
            if row1 <= row && row <= row2 && col1 <= col && col <= col2 {
                return val;
            }
        }
        self.rectangle[row as usize][col as usize]
    }
}
