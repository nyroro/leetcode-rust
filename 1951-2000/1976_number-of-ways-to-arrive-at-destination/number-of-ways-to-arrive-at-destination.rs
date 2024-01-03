
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = n as usize;
        let mut graph = vec![vec![std::i32::MAX; n]; n];
        let mut dist = vec![std::i64::MAX; n];
        let mut count = vec![0; n];
        let mut pq = std::collections::BinaryHeap::new();

        for road in roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let time = road[2];
            graph[u][v] = time;
            graph[v][u] = time;
        }

        dist[0] = 0;
        count[0] = 1;
        pq.push(std::cmp::Reverse((0, 0)));

        while let Some(std::cmp::Reverse((d, node))) = pq.pop() {
            if d > dist[node] {
                continue;
            }
            for next in 0..n {
                if graph[node][next] != std::i32::MAX {
                    let new_dist = dist[node] + graph[node][next] as i64;
                    if new_dist < dist[next] {
                        dist[next] = new_dist;
                        count[next] = count[node];
                        pq.push(std::cmp::Reverse((new_dist, next)));
                    } else if new_dist == dist[next] {
                        count[next] = (count[next] + count[node]) % MOD;
                    }
                }
            }
        }

        count[n - 1] as i32

    }
}
