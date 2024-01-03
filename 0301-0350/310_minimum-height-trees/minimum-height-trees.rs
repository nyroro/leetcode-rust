
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        
        // 创建邻接表

        let mut adj_list = vec![vec![]; n as usize];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj_list[u].push(v);
            adj_list[v].push(u);
        }
        
        // 初始化度数数组

        let mut degrees = vec![0; n as usize];
        for neighbors in &adj_list {
            for neighbor in neighbors {
                degrees[*neighbor as usize] += 1;
            }
        }
        
        // 创建队列并将度数为 1 的节点入队

        let mut queue = Vec::new();
        for (i, degree) in degrees.iter().enumerate() {
            if *degree == 1 {
                queue.push(i as i32);
            }
        }
        
        // 执行广度优先搜索

        let mut result = Vec::new();
        while !queue.is_empty() {
            result = queue.clone();
            let mut next_level = Vec::new();
            for node in &queue {
                let node = *node as usize;
                for neighbor in &adj_list[node] {
                    degrees[*neighbor as usize] -= 1;
                    if degrees[*neighbor as usize] == 1 {
                        next_level.push(*neighbor as i32);
                    }
                }
            }
            queue = next_level;
        }
        
        result

    }
}
