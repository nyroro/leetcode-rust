
impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let mut dp = vec![vec![100001; floor.len()]; (num_carpets + 1) as usize];
        
        for j in 0..floor.len() {
            dp[0][j] = if floor.chars().nth(j).unwrap() == '1' { 1 } else { 0 };
            if j > 0 {
                dp[0][j] += dp[0][j-1];
            }
        }
        
        for i in 1..=num_carpets {
            for j in 0..carpet_len as usize {
                dp[i as usize][j] = 0;
            }
            for j in carpet_len as usize..floor.len() {
                if floor.chars().nth(j).unwrap() == '1' {
                    dp[i as usize][j] = dp[(i-1) as usize][j - carpet_len as usize].min(dp[i as usize][j-1] + 1);
                } else {
                    dp[i as usize][j] = dp[i as usize][j-1];
                }
            }
        }
        
        dp[num_carpets as usize][floor.len() - 1]
    }
}
