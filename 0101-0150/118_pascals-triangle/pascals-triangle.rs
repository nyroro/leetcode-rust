
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut triangle: Vec<Vec<i32>> = Vec::new();
        
        for row in 0..num_rows {
            let mut curr_row: Vec<i32> = Vec::new();
            
            for j in 0..=row {
                if j == 0 || j == row {
                    curr_row.push(1);
                } else {
                    let prev_row = &triangle[row as usize - 1];
                    let val = prev_row[j as usize - 1] + prev_row[j as usize];
                    curr_row.push(val);
                }
            }
            
            triangle.push(curr_row);
        }
        
        triangle

    }
}
