
use std::collections::BinaryHeap;
use std::cmp::Reverse;



impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut sum = 0;
        for i in 0..m {
            sum += mat[i][0];
        }
        let mut heap = BinaryHeap::new();
        let mut visited = std::collections::HashSet::new();
        heap.push(Reverse((sum, vec![0; m])));
        visited.insert(vec![0; m]);
        for _ in 0..k-1 {
            let (s, idx) = heap.pop().unwrap().0;
            for i in 0..m {
                let mut next_idx = idx.clone();
                if next_idx[i] + 1 < n {
                    next_idx[i] += 1;
                    if visited.insert(next_idx.clone()) {
                        let mut next_sum = s - mat[i][idx[i]] + mat[i][next_idx[i]];
                        heap.push(Reverse((next_sum, next_idx)));
                    }
                }
            }
        }
        heap.pop().unwrap().0 .0

    }
}
