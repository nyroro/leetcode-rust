
use std::collections::HashMap;

impl Solution {
    pub fn relocate_marbles(nums: Vec<i32>, move_from: Vec<i32>, move_to: Vec<i32>) -> Vec<i32> {
        let mut marble_positions: HashMap<i32, i32> = HashMap::new();

        // 初始化弹珠位置和数量

        for &num in &nums {
            *marble_positions.entry(num).or_insert(0) += 1;
        }

        // 逐步更新弹珠位置和数量

        for i in 0..move_from.len() {
            let from = move_from[i];
            let to = move_to[i];

            if let Some(&count) = marble_positions.get(&from) {
                *marble_positions.entry(from).or_insert(0) -= count;
                *marble_positions.entry(to).or_insert(0) += count;
                marble_positions.remove(&from);
            }
        }

        // 将有弹珠的位置按顺序放入数组中并返回

        let mut result: Vec<i32> = marble_positions.keys().cloned().collect();
        result.sort();
        result

    }
}
