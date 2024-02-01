


impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let mut dp = vec![vec![vec![-1; 301]; 27]; 27];
        return Solution::dfs(&word, 26, 26, 0, &mut dp);
    }

    fn dfs(word: &String, left: usize, right: usize, pos: usize, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if pos >= word.len() {
            return 0;
        }
        if dp[left][right][pos] == -1 {
            let to = (word.as_bytes()[pos] - b'A') as usize;
            dp[left][right][pos] = std::cmp::min(Solution::cost(left, to) + Solution::dfs(word, to, right, pos + 1, dp), 
                                                 Solution::cost(right, to) + Solution::dfs(word, left, to, pos + 1, dp));
        }
        return dp[left][right][pos];
    }

    fn cost(from: usize, to: usize) -> i32 {
        if from == 26 {
            return 0;
        }
        let from_x = from / 6;
        let from_y = from % 6;
        let to_x = to / 6;
        let to_y = to % 6;
        return (from_x as i32 - to_x as i32).abs() + (from_y as i32 - to_y as i32).abs();
    }
}
