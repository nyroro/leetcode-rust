
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_map = std::collections::HashMap::new();
        let mut count = 0;

        for row in &grid {
            *row_map.entry(row).or_insert(0) += 1;
        }

        for col in 0..grid.len() {
            let mut col_vec = Vec::new();
            for row in &grid {
                col_vec.push(row[col]);
            }
            count += row_map.get(&col_vec).unwrap_or(&0);
        }

        count

    }
}
