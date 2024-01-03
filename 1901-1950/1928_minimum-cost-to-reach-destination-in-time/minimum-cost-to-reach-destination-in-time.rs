
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    time: i32,
    city: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.time.cmp(&other.time))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        let n = passing_fees.len();
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for edge in edges {
            let (x, y, time) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[x].push((y, time));
            graph[y].push((x, time));
        }

        let mut min_cost: Vec<Vec<i32>> = vec![vec![i32::MAX; max_time as usize + 1]; n];
        let mut pq = BinaryHeap::new();
        pq.push(State { cost: passing_fees[0], time: 0, city: 0 });
        min_cost[0][0] = passing_fees[0];

        while let Some(State { cost, time, city }) = pq.pop() {
            if city == n - 1 {
                return cost;
            }
            if time > max_time {
                continue;
            }
            if cost > min_cost[city][time as usize] {
                continue;
            }
            for &(next, next_time) in &graph[city] {
                let next_cost = cost + passing_fees[next];
                let next_time = time + next_time;
                if next_time <= max_time && next_cost < min_cost[next][next_time as usize] {
                    min_cost[next][next_time as usize] = next_cost;
                    pq.push(State { cost: next_cost, time: next_time, city: next });
                }
            }
        }
        -1

    }
}
