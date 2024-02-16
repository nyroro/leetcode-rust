
use std::collections::BinaryHeap;
use std::cmp::Reverse;



impl Solution {
    pub fn minimum_cost(start: Vec<i32>, target: Vec<i32>, special_roads: Vec<Vec<i32>>) -> i32 {
        const INF: i32 = 1e9 as i32 + 10;
        let n = special_roads.len();

        let mut d = vec![INF; n];
        let mut pq = BinaryHeap::new();

        for i in 0..n {
            d[i] = (start[0] - special_roads[i][0]).abs() + (start[1] - special_roads[i][1]).abs() + special_roads[i][4];
            pq.push((Reverse(d[i]), i));
        }

        let mut ans = (start[0] - target[0]).abs() + (start[1] - target[1]).abs();

        while let Some((Reverse(d_c), c)) = pq.pop() {
            if d_c != d[c] {
                continue;
            }

            ans = ans.min(d_c + (target[0] - special_roads[c][2]).abs() + (target[1] - special_roads[c][3]).abs());

            for nxt in 0..n {
                let w = (special_roads[c][2] - special_roads[nxt][0]).abs() + (special_roads[c][3] - special_roads[nxt][1]).abs() + special_roads[nxt][4];
                if d_c + w < d[nxt] {
                    d[nxt] = d_c + w;
                    pq.push((Reverse(d[nxt]), nxt));
                }
            }
        }

        ans

    }
}
