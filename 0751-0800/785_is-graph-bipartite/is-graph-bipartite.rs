
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut colors = vec![0; n]; // 用于记录节点的颜色，初始值为 0


        fn dfs(node: usize, color: i32, graph: &Vec<Vec<i32>>, colors: &mut Vec<i32>) -> bool {
            colors[node] = color; // 将当前节点染色


            for &neighbor in &graph[node] {
                if colors[neighbor as usize] == color {
                    return false; // 如果相邻节点已经染成了相同的颜色，返回 false

                }
                if colors[neighbor as usize] == 0 && !dfs(neighbor as usize, -color, graph, colors) {
                    return false; // 递归染色相邻节点，并判断是否满足条件

                }
            }

            true

        }

        for i in 0..n {
            if colors[i] == 0 && !dfs(i, 1, &graph, &mut colors) {
                return false; // 对每个未染色的节点进行 DFS

            }
        }

        true

    }
}
