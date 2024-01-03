
impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        satisfaction.sort_unstable();
        let mut sum = 0;
        let mut total = 0;
        for i in (0..satisfaction.len()).rev() {
            if sum + satisfaction[i] > 0 {
                sum += satisfaction[i];
                total += sum;
            } else {
                break;
            }
        }
        total

    }
}
