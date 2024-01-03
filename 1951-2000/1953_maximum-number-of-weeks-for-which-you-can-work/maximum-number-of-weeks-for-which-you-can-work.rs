
impl Solution {
    pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
        let mut max_milestones = 0;
        let mut total_milestones = 0;

        for milestone in &milestones {
            max_milestones = max_milestones.max(*milestone);
            total_milestones += *milestone as i64;
        }

        let remaining_milestones = total_milestones - max_milestones as i64;

        if remaining_milestones >= max_milestones as i64 {
            total_milestones

        } else {
            2 * remaining_milestones + 1

        }
    }
}
