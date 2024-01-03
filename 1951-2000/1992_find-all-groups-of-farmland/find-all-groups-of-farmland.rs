
impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let m = land.len();
        let n = land[0].len();
        
        fn dfs(land: &mut Vec<Vec<i32>>, r: usize, c: usize, result: &mut Vec<Vec<i32>>) {
            if r >= land.len() || c >= land[0].len() || land[r][c] != 1 {
                return;
            }
            
            let mut r2 = r;
            let mut c2 = c;
            
            while r2 < land.len() && land[r2][c] == 1 {
                r2 += 1;
            }
            
            while c2 < land[0].len() && land[r][c2] == 1 {
                c2 += 1;
            }
            
            result.push(vec![r as i32, c as i32, r2 as i32 - 1, c2 as i32 - 1]);
            
            for i in r..r2 {
                for j in c..c2 {
                    land[i][j] = -1;
                }
            }
            
            dfs(land, r2, c, result);
            dfs(land, r, c2, result);
        }
        
        let mut mutable_land = land.clone();
        
        for i in 0..m {
            for j in 0..n {
                if mutable_land[i][j] == 1 {
                    dfs(&mut mutable_land, i, j, &mut result);
                }
            }
        }
        
        result

    }
}
