
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits.into_iter()).collect();
        projects.sort_unstable_by_key(|&(c, _)| c);

        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        let mut available_capital = w;
        let mut completed_projects = 0;

        for _ in 0..k {
            while let Some(&(c, p)) = projects.first() {
                if c <= available_capital {
                    max_heap.push(p);
                    projects.remove(0);
                } else {
                    break;
                }
            }

            if let Some(max_profit) = max_heap.pop() {
                available_capital += max_profit;
                completed_projects += 1;
            } else {
                break;
            }
        }

        available_capital

    }
}
