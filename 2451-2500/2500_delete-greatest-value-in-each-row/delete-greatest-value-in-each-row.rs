
impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut answer = 0;
        let mut new_grid = grid.clone();

        while !new_grid.is_empty() {
            for row in new_grid.iter_mut() {
                if let Some(max_val) = row.iter().max() {
                    let max_index = row.iter().position(|&x| x == *max_val).unwrap();
                    answer += *max_val;
                    row.remove(max_index);
                }
            }
            new_grid = new_grid.into_iter().filter(|row| !row.is_empty()).collect();
        }

        answer

    }
}
