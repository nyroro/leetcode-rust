
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut tower = vec![vec![0.0; query_row as usize + 1]; query_row as usize + 1];
        tower[0][0] = poured as f64;
        
        for i in 0..query_row {
            for j in 0..=i {
                let q = (tower[i as usize][j as usize] - 1.0) / 2.0;
                if q > 0.0 {
                    tower[(i+1) as usize][j as usize] += q;
                    tower[(i+1) as usize][(j+1) as usize] += q;
                }
            }
        }
        
        tower[query_row as usize][query_glass as usize].min(1.0)
    }
}
