
impl Solution {
    pub fn min_operations_max_profit(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32 {
        let mut run = 0;
        let mut max_run = 1;
        let mut prof = 0;
        let mut max_prof = prof;
        let mut sum = 0;
        let mut i = 0;
        while sum > 0 || i < customers.len() {
            if i < customers.len() {
                sum += customers[i];
                i += 1;
            }
            let boarding = std::cmp::min(4, sum);
            sum -= boarding;
            prof = prof + boarding * boarding_cost - running_cost;
            run += 1;
            if prof > max_prof {
                max_prof = prof;
                max_run = run;
            }
        }
        return if max_prof > 0 { max_run } else { -1 };
    }
}
