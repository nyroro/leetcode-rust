
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut weakest_rows: Vec<i32> = Vec::new();
        let mut rows: Vec<(i32, i32)> = Vec::new();

        for (i, row) in mat.iter().enumerate() {
            let soldiers = row.iter().sum();
            rows.push((soldiers, i as i32));
        }

        rows.sort();

        for i in 0..k {
            weakest_rows.push(rows[i as usize].1);
        }

        weakest_rows

    }
}
