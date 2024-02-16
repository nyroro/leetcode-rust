
impl Solution {
    fn is_possible(n: usize, k: usize, mid: i64, budget: i32, composition: &Vec<i32>, stock: &Vec<i32>, cost: &Vec<i32>) -> bool {
        let mut budget = budget as i64;
        for i in 0..n {
            let needed = composition[i] as i64 * mid - stock[i] as i64;
            if needed > 0 {
                let required_budget = needed * cost[i] as i64;
                if required_budget > budget {
                    return false;
                } else {
                    budget -= required_budget;
                }
            }
        }
        true

    }

    fn solve(n: usize, k: usize, budget: i32, composition: &Vec<i32>, stock: &Vec<i32>, cost: &Vec<i32>) -> i64 {
        let mut left = 0;
        let mut right = 1e9 as i64 / n as i64;
        let mut ans = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::is_possible(n, k, mid, budget, composition, stock, cost) {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans

    }

    pub fn max_number_of_alloys(n: i32, k: i32, budget: i32, composition: Vec<Vec<i32>>, stock: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..k as usize {
            ans = ans.max(Self::solve(n as usize, k as usize, budget, &composition[i], &stock, &cost));
        }
        ans as i32

    }
}
