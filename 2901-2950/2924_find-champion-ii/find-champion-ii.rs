
impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut indegree = vec![0; n as usize];
        let mut graph = vec![vec![]; n as usize];
        
        // 构建图和计算入度

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            indegree[v] += 1;
        }
        
        // 找到没有入度的节点

        let mut champions = Vec::new();
        for i in 0..n as usize {
            if indegree[i] == 0 {
                champions.push(i as i32);
            }
        }
        
        // 如果没有唯一的冠军团队，返回 -1

        if champions.len() != 1 {
            return -1;
        }
        
        champions[0]
    }
}
