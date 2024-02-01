


impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let mut ans = 0;
        let n = roads.len() + 1;
        let mut graph: Vec<Vec<i32>> = vec![vec![]; n];
        
        // 构建图

        for road in &roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            graph[u].push(v as i32);
            graph[v].push(u as i32);
        }
        
        // DFS函数

        fn dfs(graph: &Vec<Vec<i32>>, u: usize, prev: i32, seats: i32, ans: &mut i64) -> i32 {
            let mut people = 1;
            for &v in &graph[u] {
                if v == prev {
                    continue;
                }
                people += dfs(graph, v as usize, u as i32, seats, ans);
            }
            if u > 0 {
                // 计算所需汽车数量 = 向上取整(people / seats)
                *ans += (people + seats - 1) / seats;
            }
            people

        }
        
        dfs(&graph, 0, -1, seats, &mut ans);
        ans

    }
}
