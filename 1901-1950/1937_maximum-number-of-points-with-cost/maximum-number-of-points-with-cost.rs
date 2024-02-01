
use std::collections::BinaryHeap;
use std::cmp::Reverse;



impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i64 {
        let n = points.len();
        let m = points[0].len();
        let mut dp: Vec<i64> = points[0].iter().map(|&x| x as i64).collect();

        for i in 1..n {
            let mut nxt = vec![0; m];
            let mut max_heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();

            for j in 0..m {
                nxt[j] = points[i][j] as i64 + dp[j];
            }

            for j in (0..m).rev() {
                if let Some(max_val) = max_heap.peek() {
                    nxt[j] = nxt[j].max(-(max_val.0) + j as i64 + points[i][j] as i64);
                }
                max_heap.push(Reverse(-dp[j] + j as i64));
            }

            max_heap.clear();

            for j in 0..m {
                if let Some(max_val) = max_heap.peek() {
                    nxt[j] = nxt[j].max(-(max_val.0) + points[i][j] as i64 - j as i64);
                }
                max_heap.push(Reverse(-dp[j] - j as i64));
            }

            dp = nxt;
        }

        *dp.iter().max().unwrap()
    }
}
