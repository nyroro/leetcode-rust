
impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot;
        let mut factory = factory;
        
        robot.sort();
        factory.sort();
        
        let m = robot.len();
        let n = factory.len();
        
        let mut dp = vec![vec![0; n + 1]; m + 1];
        
        for i in 0..m {
            dp[i][n] = std::i64::MAX;
        }
        
        for j in (0..n).rev() {
            let mut prefix = 0;
            let mut qq = std::collections::VecDeque::new();
            qq.push_back((m, 0));
            
            for i in (0..m).rev() {
                prefix += (robot[i] as i64 - factory[j][0] as i64).abs();
                
                if qq.front().unwrap().0 > i + factory[j][1] as usize {
                    qq.pop_front();
                }
                
                while let Some(last) = qq.back() {
                    if last.1 >= dp[i][j + 1] - prefix {
                        qq.pop_back();
                    } else {
                        break;
                    }
                }
                
                qq.push_back((i, dp[i][j + 1] - prefix));
                dp[i][j] = qq.front().unwrap().1 + prefix;
            }
        }
        
        dp[0][0]
    }
}
