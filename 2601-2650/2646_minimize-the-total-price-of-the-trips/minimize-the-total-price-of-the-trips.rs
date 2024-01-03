
use std::collections::HashMap;

impl Solution {
    pub fn minimum_total_price(n: i32, edges: Vec<Vec<i32>>, price: Vec<i32>, trips: Vec<Vec<i32>>) -> i32 {
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut cost: Vec<i32> = vec![0; n as usize];
        let mut dp: Vec<Vec<Option<i32>>> = vec![vec![None; 2]; n as usize];
        
        for item in edges {
            adj.entry(item[0]).or_insert(vec![]).push(item[1]);
            adj.entry(item[1]).or_insert(vec![]).push(item[0]);
        }
        
        for item in &trips {
            let mut path = vec![];
            Solution::find(item[0], &mut path, &mut cost, item[1], &adj, -1);
        }
        
        for i in 0..cost.len() {
            cost[i] *= price[i];
        }
        
        Solution::dfs(&cost, 0, &adj, &mut dp, -1, false)
    }
    
    fn find(node: i32, path: &mut Vec<i32>, cost: &mut Vec<i32>, end: i32, adj: &HashMap<i32, Vec<i32>>, p: i32) {
        path.push(node);
        if end == node {
            for item in path.iter() {
                cost[*item as usize] += 1;
            }
            path.pop();
            return;
        }
        if let Some(neighbors) = adj.get(&node) {
            for &next in neighbors {
                if next != p {
                    Solution::find(next, path, cost, end, adj, node);
                }
            }
        }
        path.pop();
    }
    
    fn dfs(cost: &Vec<i32>, node: i32, adj: &HashMap<i32, Vec<i32>>, dp: &mut Vec<Vec<Option<i32>>>, p: i32, p_half: bool) -> i32 {
        let mut half_cost = cost[node as usize] / 2;
        let mut full_cost = cost[node as usize];
        if let Some(cached) = dp[node as usize][p_half as usize] {
            return cached;
        }
        if let Some(neighbors) = adj.get(&node) {
            for &next in neighbors {
                if next != p {
                    full_cost += Solution::dfs(cost, next, adj, dp, node, false);
                }
            }
        }
        if p_half {
            dp[node as usize][p_half as usize] = Some(full_cost);
            return full_cost;
        }
        if let Some(neighbors) = adj.get(&node) {
            for &next in neighbors {
                if next != p {
                    half_cost += Solution::dfs(cost, next, adj, dp, node, true);
                }
            }
        }
        dp[node as usize][p_half as usize] = Some(full_cost.min(half_cost));
        full_cost.min(half_cost)
    }
}
