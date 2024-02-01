


impl Solution {
    pub fn number_of_sets(n: i32, max_distance: i32, roads: Vec<Vec<i32>>) -> i32 {
        let inf = 1000000007;
        let n = n as usize; // Convert n to usize for indexing

        let mut dist = vec![vec![inf; n]; n]; // Use usize for vector size


        for road in &roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let w = road[2];
            dist[u][v] = dist[u][v].min(w);
            dist[v][u] = dist[v][u].min(w);
        }

        let mut ans = 0;
        for mask in 0..(1 << n) {
            let mut new_dist = dist.clone();
            let mut rem = Vec::new();
            for i in 0..n {
                if (mask >> i) & 1 == 0 {
                    for j in 0..n {
                        new_dist[i][j] = inf;
                        new_dist[j][i] = inf;
                    }
                } else {
                    rem.push(i);
                }
            }

            let mut valid = true;
            for k in 0..n {
                for i in 0..n {
                    for j in 0..n {
                        if new_dist[i][k] < inf && new_dist[k][j] < inf {
                            new_dist[i][j] = new_dist[i][j].min(new_dist[i][k] + new_dist[k][j]);
                        }
                    }
                }
            }

            for &x in &rem {
                for &y in &rem {
                    if x != y && new_dist[x][y] > max_distance {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }

            if valid {
                ans += 1;
            }
        }

        ans

    }
}
