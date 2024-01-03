
impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut closest = i32::MAX;
        for &base in &base_costs {
            Self::dfs(&topping_costs, target, base, 0, &mut closest);
        }
        closest

    }
    
    fn dfs(topping_costs: &[i32], target: i32, cost: i32, index: usize, closest: &mut i32) {
        if (cost - target).abs() < (*closest - target).abs() || (cost - target).abs() == (*closest - target).abs() && cost < *closest {
            *closest = cost;
        }
        if index == topping_costs.len() || cost >= target {
            return;
        }
        for i in index..topping_costs.len() {
            Self::dfs(topping_costs, target, cost, i + 1, closest);
            Self::dfs(topping_costs, target, cost + topping_costs[i], i + 1, closest);
            Self::dfs(topping_costs, target, cost + 2 * topping_costs[i], i + 1, closest);
        }
    }
}
