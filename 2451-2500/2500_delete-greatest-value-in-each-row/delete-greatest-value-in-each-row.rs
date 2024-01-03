
impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut answer = 0;
        let mut new_grid = grid.clone();

        while !new_grid.is_empty() {
            let mut max_values = Vec::new();
            for row in new_grid.iter_mut() {
                if let Some(max_val) = row.iter().max() {
                    let max_index = row.iter().position(|&x| x == *max_val).unwrap();
                    max_values.push(*max_val);
                    row.remove(max_index);
                }
            }
            answer += max_values.iter().max().unwrap();
            new_grid = new_grid.into_iter().filter(|row| !row.is_empty()).collect();
        }

        answer

    }
}
