
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let mut s1: Vec<i32> = vec![0; n];
        let mut s2: Vec<i32> = vec![0; n];
        
        s1[0] = grid[0][0];
        s2[0] = grid[1][0];
        
        for i in 1..n {
            s1[i] = grid[0][i] + s1[i-1];
            s2[i] = grid[1][i] + s2[i-1];
        }
        
        let mut arr: Vec<i32> = vec![0; n];
        
        for i in 0..n {
            let k1 = s1[n-1] - s1[i];
            let k2 = if i > 0 { s2[i-1] } else { 0 };
            arr[i] = k1.max(k2);
        }
        
        *arr.iter().min().unwrap() as i64

    }
}
