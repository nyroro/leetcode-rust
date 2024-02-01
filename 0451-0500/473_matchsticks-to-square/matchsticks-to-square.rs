
impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let total_length: i32 = matchsticks.iter().sum();
        if total_length % 4 != 0 {
            return false;
        }
        let target: i32 = total_length / 4;
        let mut sides: [i32; 4] = [0; 4];
        return Self::can_make_square(&matchsticks, &mut sides, 0, target);
    }

    fn can_make_square(matchsticks: &Vec<i32>, sides: &mut [i32; 4], index: usize, target: i32) -> bool {
        if index == matchsticks.len() {
            return sides[0] == sides[1] && sides[1] == sides[2] && sides[2] == sides[3];
        }
        for i in 0..4 {
            if sides[i] + matchsticks[index] > target {
                continue;
            }
            sides[i] += matchsticks[index];
            if Self::can_make_square(matchsticks, sides, index + 1, target) {
                return true;
            }
            sides[i] -= matchsticks[index];
        }
        return false;
    }
}
