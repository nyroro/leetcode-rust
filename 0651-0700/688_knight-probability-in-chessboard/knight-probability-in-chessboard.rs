
impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let directions: [(i32, i32); 8] = [
            (-2, -1), (-2, 1), (-1, -2), (-1, 2),
            (1, -2), (1, 2), (2, -1), (2, 1)
        ];
        
        let mut dp_prev = vec![vec![0.0; n as usize]; n as usize];
        dp_prev[row as usize][column as usize] = 1.0;
        
        for _ in 0..k {
            let mut dp_curr = vec![vec![0.0; n as usize]; n as usize];
            
            for i in 0..n {
                for j in 0..n {
                    for &(dx, dy) in &directions {
                        let x = i + dx;
                        let y = j + dy;
                        
                        if x >= 0 && x < n && y >= 0 && y < n {
                            dp_curr[x as usize][y as usize] += dp_prev[i as usize][j as usize] / 8.0;
                        }
                    }
                }
            }
            
            dp_prev = dp_curr;
        }
        
        let mut probability = 0.0;
        
        for i in 0..n {
            for j in 0..n {
                probability += dp_prev[i as usize][j as usize];
            }
        }
        
        probability

    }
}
