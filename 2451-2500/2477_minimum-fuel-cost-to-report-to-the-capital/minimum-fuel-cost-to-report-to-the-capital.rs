


impl Solution {
    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len() + 1;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        
        // 构建图的邻接表表示

        for road in &roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        
        // 深度优先搜索函数

        fn dfs(u: usize, parent: usize, graph: &Vec<Vec<usize>>, seats: i32) -> (i64, i32) {
            let mut total_fuel = 0;
            let mut total_people = 1;
            for &v in &graph[u] {
                if v == parent {
                    continue;
                }
                let (fuel, people) = dfs(v, u, graph, seats);
                total_fuel += fuel + ((people + seats - 1) as i64 / seats as i64);
                total_people += people;
            }
            (total_fuel, total_people)
        }
        
        let (fuel, _) = dfs(0, n, &graph, seats);
        fuel

    }
}
