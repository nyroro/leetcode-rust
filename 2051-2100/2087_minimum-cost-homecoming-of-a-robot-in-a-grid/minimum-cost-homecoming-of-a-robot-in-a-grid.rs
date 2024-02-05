
impl Solution {
    pub fn min_cost(start_pos: Vec<i32>, home_pos: Vec<i32>, row_costs: Vec<i32>, col_costs: Vec<i32>) -> i32 {
        let (sx, sy) = (start_pos[0] as usize, start_pos[1] as usize);
        let (hx, hy) = (home_pos[0] as usize, home_pos[1] as usize);

        let min_row = std::cmp::min(sx, hx);
        let max_row = std::cmp::max(sx, hx);
        let min_col = std::cmp::min(sy, hy);
        let max_col = std::cmp::max(sy, hy);

        let total_row_cost: i32 = row_costs[min_row..=max_row].iter().sum();
        let total_col_cost: i32 = col_costs[min_col..=max_col].iter().sum();

        total_row_cost + total_col_cost - row_costs[sx] - col_costs[sy]
    }
}
