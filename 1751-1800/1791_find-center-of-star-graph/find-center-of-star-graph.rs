
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        // 创建一个HashMap来存储节点和其连接次数

        let mut node_connections = std::collections::HashMap::new();
        
        // 遍历边的数组

        for edge in &edges {
            // 对于每条边，将连接次数加1

            *node_connections.entry(edge[0]).or_insert(0) += 1;
            *node_connections.entry(edge[1]).or_insert(0) += 1;
        }
        
        // 找到连接次数最多的节点

        let center_node = node_connections.iter().max_by_key(|&(_, count)| count).unwrap().0;
        
        // 返回中心节点

        *center_node

    }
}
