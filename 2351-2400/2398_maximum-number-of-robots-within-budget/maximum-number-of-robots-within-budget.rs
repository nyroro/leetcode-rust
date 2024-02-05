
use std::collections::VecDeque;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let mut charge_time_stack: VecDeque<(i64, usize)> = VecDeque::new();
        let mut running_sum = 0;
        let (mut left, mut right) = (0, 0);
        let mut ret = 0;
        let n = charge_times.len();

        while right < n {
            running_sum += running_costs[right as usize] as i64;
            while !charge_time_stack.is_empty() && charge_time_stack.back().unwrap().0 < charge_times[right as usize] as i64 {
                charge_time_stack.pop_back();
            }
            charge_time_stack.push_back((charge_times[right as usize] as i64, right as usize));
            let mut cost = (right - left + 1) as i64 * running_sum + charge_time_stack.front().unwrap().0;
            while cost > budget && left <= right {
                running_sum -= running_costs[left as usize] as i64;
                if charge_time_stack.front().unwrap().1 == left {
                    charge_time_stack.pop_front();
                }
                left += 1;
                cost = ((right - left + 1) as i64 * running_sum + charge_time_stack.front().unwrap_or(&(0, 0)).0);
            }
            ret = ret.max(right - left + 1);
            right += 1;
        }

        ret as i32

    }
}
