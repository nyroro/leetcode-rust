
impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut max_ones = 0;
        let mut max_ones_row = 0;
        
        for (i, row) in mat.iter().enumerate() {
            let ones_count = row.iter().filter(|&&x| x == 1).count() as i32;
            if ones_count > max_ones {
                max_ones = ones_count;
                max_ones_row = i as i32;
            }
        }
        
        vec![max_ones_row, max_ones]
    }
}
