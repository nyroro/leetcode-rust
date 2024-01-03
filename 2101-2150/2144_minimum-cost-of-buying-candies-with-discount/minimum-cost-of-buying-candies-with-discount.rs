
impl Solution {
    pub fn minimum_cost(cost: Vec<i32>) -> i32 {
        let mut sorted_cost = cost.clone();
        sorted_cost.sort();

        let mut total_cost = 0;
        let mut i = sorted_cost.len() as i32 - 1;

        while i >= 0 {
            if i - 1 >= 0 {
                total_cost += sorted_cost[i as usize] + sorted_cost[(i - 1) as usize];
                i -= 3;
            } else {
                total_cost += sorted_cost[i as usize];
                i -= 1;
            }
        }

        total_cost

    }
}
