
impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut diff_costs: Vec<(i32, usize)> = Vec::new();
        let n = costs.len() / 2;
        let mut total_cost = 0;

        for i in 0..costs.len() {
            let diff = costs[i][0] - costs[i][1];
            diff_costs.push((diff, i));
        }

        diff_costs.sort();

        for i in 0..n {
            total_cost += costs[diff_costs[i].1][0];
        }

        for i in n..2 * n {
            total_cost += costs[diff_costs[i].1][1];
        }

        total_cost

    }
}
