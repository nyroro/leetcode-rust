
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut visited = vec![false; n];
        let mut min_cost = 0;
        let mut pq = std::collections::BinaryHeap::new();
        
        visited[0] = true;
        for i in 1..n {
            let cost = Self::manhattan_distance(&points[0], &points[i]);
            pq.push(std::cmp::Reverse((cost, i)));
        }
        
        while let Some(std::cmp::Reverse((cost, curr))) = pq.pop() {
            if visited[curr] {
                continue;
            }
            visited[curr] = true;
            min_cost += cost;
            
            for i in 0..n {
                if !visited[i] {
                    let cost = Self::manhattan_distance(&points[curr], &points[i]);
                    pq.push(std::cmp::Reverse((cost, i)));
                }
            }
        }
        
        min_cost

    }
    
    fn manhattan_distance(p1: &[i32], p2: &[i32]) -> i32 {
        (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
    }
}
