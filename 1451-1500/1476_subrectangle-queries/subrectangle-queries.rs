
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

fn main() {
    // 测试代码

    let rectangle = vec![
        vec![1, 2, 1],
        vec![4, 3, 4],
        vec![3, 2, 1],
        vec![1, 1, 1],
    ];
    let mut obj = SubrectangleQueries::new(rectangle);
    assert_eq!(obj.get_value(0, 2), 1);
    obj.update_subrectangle(0, 0, 3, 2, 5);
    assert_eq!(obj.get_value(0, 2), 5);
    assert_eq!(obj.get_value(3, 1), 5);
    obj.update_subrectangle(3, 0, 3, 2, 10);
    assert_eq!(obj.get_value(3, 1), 10);
    assert_eq!(obj.get_value(0, 2), 5);
}
