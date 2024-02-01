


impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let mut memo = vec![vec![vec![-1; n as usize + 1]; target as usize + 1]; m as usize];
        let int_max = 100000001;
        
        let res = Solution::min_cost_util(&mut memo, &houses, &cost, m, n, target, 0, 0, int_max);
        if res == int_max {
            return -1;
        }
        return res;
    }
    
    fn min_cost_util(memo: &mut Vec<Vec<Vec<i32>>, houses: &Vec<i32>, cost: &Vec<Vec<i32>>, m: i32, n: i32, target: i32, i: i32, previous: i32, int_max: i32) -> i32 {
        if i == m {
            if target == 0 {
                return 0;
            }
            return int_max;
        }
        if target < 0 {
            return int_max;
        }
        
        if memo[i as usize][target as usize][previous as usize] != -1 {
            return memo[i as usize][target as usize][previous as usize];
        }
        
        let mut ans = int_max;
        
        if houses[i as usize] == 0 {
            for j in 1..=n {
                ans = ans.min(cost[i as usize][(j - 1) as usize] + Solution::min_cost_util(memo, houses, cost, m, n, target - if j != previous { 1 } else { 0 }, i + 1, j, int_max));
            }
        } else {
            ans = Solution::min_cost_util(memo, houses, cost, m, n, target - if houses[i as usize] != previous { 1 } else { 0 }, i + 1, houses[i as usize], int_max);
        }
        
        memo[i as usize][target as usize][previous as usize] = ans;
        return ans;
    }
}
