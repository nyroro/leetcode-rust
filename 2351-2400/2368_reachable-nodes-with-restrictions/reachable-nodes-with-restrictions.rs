
// 定义树的节点结构体

#[derive(Clone)]  // 添加 #[derive(Clone)] 注解

struct TreeNode {
    node: i32,
    neighbors: Vec<i32>,
}

impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        // 构建邻接表表示树的结构

        let mut tree: Vec<TreeNode> = vec![TreeNode { node: 0, neighbors: vec![] }; n as usize];
        for edge in edges.iter() {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            tree[a].neighbors.push(b as i32);
            tree[b].neighbors.push(a as i32);
        }
        
        // 使用深度优先搜索（DFS）来计算可到达的节点数

        let mut visited = vec![false; n as usize];
        let mut result = 0;
        let mut stack = vec![(0, n)];
        while let Some((node, remaining)) = stack.pop() {
            if visited[node as usize] {
                continue;
            }
            visited[node as usize] = true;
            result += 1;
            for &neighbor in tree[node as usize].neighbors.iter() {
                if !visited[neighbor as usize] && !restricted.contains(&neighbor) && remaining > 0 {
                    stack.push((neighbor, remaining - 1));
                }
            }
        }
        
        result

    }
}
