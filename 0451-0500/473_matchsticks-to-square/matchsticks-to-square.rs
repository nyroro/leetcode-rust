
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let total_length: i32 = matchsticks.iter().sum();
        if total_length % 4 != 0 {
            return false;
        }
        let target: i32 = total_length / 4;
        let n: usize = matchsticks.len();
        let mut used: Vec<bool> = vec![false; n];
        return Self::backtrack(&matchsticks, &mut used, 0, 0, 0, target);
    }
    
    fn backtrack(matchsticks: &Vec<i32>, used: &mut Vec<bool>, sides: i32, current: i32, index: usize, target: i32) -> bool {
        if sides == 4 {
            return true;
        }
        if current == target {
            return Self::backtrack(matchsticks, used, sides + 1, 0, 0, target);
        }
        for i in index..matchsticks.len() {
            if used[i] || current + matchsticks[i] > target {
                continue;
            }
            used[i] = true;
            if Self::backtrack(matchsticks, used, sides, current + matchsticks[i], i + 1, target) {
                return true;
            }
            used[i] = false;
        }
        false

    }
}
