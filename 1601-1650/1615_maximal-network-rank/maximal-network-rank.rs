
impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        // 创建二维数组表示城市之间的连接关系

        let mut adjacency = vec![vec![0; n as usize]; n as usize];
        
        // 遍历所有的道路，更新连接关系

        for road in roads {
            let city1 = road[0] as usize;
            let city2 = road[1] as usize;
            adjacency[city1][city2] += 1;
            adjacency[city2][city1] += 1;
        }
        
        let mut max_rank = 0;
        
        // 计算每对不同城市之间的网络等级，并更新最大网络等级

        for i in 0..n as usize {
            for j in (i + 1)..n as usize {
                let rank = adjacency[i].iter().sum::<i32>() + adjacency[j].iter().sum::<i32>() - adjacency[i][j];
                max_rank = max_rank.max(rank);
            }
        }
        
        max_rank

    }
}
