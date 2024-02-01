
impl Solution {
    fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 创建一个空的二维数组用于存储关键连接

        let mut critical_connections = Vec::new();
        
        // 创建辅助数据结构

        let mut ids = vec![-1; n as usize];
        let mut low = vec![0; n as usize];
        let mut visited = vec![false; n as usize];
        let mut adj = vec![Vec::new(); n as usize];
        
        // 构建连接关系图

        for connection in connections {
            let u = connection[0] as usize;
            let v = connection[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }
        
        // 定义DFS函数来查找关键连接

        fn dfs(u: usize, parent: usize, id: &mut i32, ids: &mut Vec<i32>, low: &mut Vec<i32>, visited: &mut Vec<bool>, adj: &Vec<Vec<usize>>, critical_connections: &mut Vec<Vec<i32>>) {
            visited[u] = true;
            ids[u] = *id;
            low[u] = *id;
            *id += 1;
            
            for &v in &adj[u] {
                if v == parent {
                    continue;
                }
                
                if !visited[v] {
                    dfs(v, u, id, ids, low, visited, adj, critical_connections);
                    low[u] = low[u].min(low[v]);
                    
                    if ids[u] < low[v] {
                        critical_connections.push(vec![u as i32, v as i32]);
                    }
                } else {
                    low[u] = low[u].min(ids[v]);
                }
            }
        }
        
        // 调用DFS函数查找关键连接

        let mut id = 0;
        dfs(0, 0, &mut id, &mut ids, &mut low, &mut visited, &adj, &mut critical_connections);
        
        // 返回找到的关键连接

        critical_connections

    }
}
