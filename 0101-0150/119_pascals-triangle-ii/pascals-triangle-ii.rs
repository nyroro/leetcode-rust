
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut row = vec![0; (row_index + 1) as usize];
        row[0] = 1;
        
        for i in 1..=row_index as usize {
            for j in (1..=i).rev() {
                row[j] += row[j - 1];
            }
        }
        
        row

    }
}
