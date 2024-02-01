
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        // 初始化dist数组

        let mut dist = vec![vec![i32::MAX; n as usize]; n as usize];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let weight = edge[2];
            dist[u][v] = weight;
            dist[v][u] = weight;
        }
        
        // 使用Floyd-Warshall算法更新dist数组

        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    if dist[i][j] > dist[i][k] + dist[k][j] {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }
        
        let mut min_cities = n;
        let mut result = 0;
        
        // 统计满足条件的城市数量，并找到具有最大数量的城市编号

        for i in 0..n as usize {
            let mut count = 0;
            for j in 0..n as usize {
                if i != j && dist[i][j] <= distance_threshold {
                    count += 1;
                }
            }
            if count <= min_cities {
                min_cities = count;
                result = i as i32;
            }
        }
        
        result

    }
}
