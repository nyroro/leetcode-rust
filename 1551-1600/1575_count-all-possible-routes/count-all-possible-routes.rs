


impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let n = locations.len();
        let mut dp = vec![vec![vec![-1; 201]; n]; n];
        
        fn dfs(locations: &Vec<i32>, start: usize, finish: usize, fuel: i32, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
            let n = locations.len();
            if dp[start][finish][fuel as usize] != -1 {
                return dp[start][finish][fuel as usize];
            }
            let mut ans = if start == finish { 1 } else { 0 };
            for i in 0..n {
                if i != start {
                    let dist = (locations[start] - locations[i]).abs() as i32;
                    if fuel >= dist {
                        ans = (ans + dfs(locations, i, finish, fuel - dist, dp)) % 1000000007;
                    }
                }
            }
            dp[start][finish][fuel as usize] = ans;
            ans

        }
        
        dfs(&locations, start as usize, finish as usize, fuel, &mut dp)
    }
}
