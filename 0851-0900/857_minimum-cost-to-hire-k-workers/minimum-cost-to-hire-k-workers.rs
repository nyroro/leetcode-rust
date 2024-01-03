
impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let n = quality.len();
        let mut workers: Vec<(f64, i32)> = Vec::new();
        for i in 0..n {
            workers.push((wage[i] as f64 / quality[i] as f64, quality[i]));
        }
        workers.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        
        let mut total_quality = 0;
        let mut min_cost = f64::MAX;
        let mut max_heap: std::collections::BinaryHeap<i32> = std::collections::BinaryHeap::new();
        
        for (ratio, q) in workers {
            total_quality += q;
            max_heap.push(q);
            if max_heap.len() as i32 > k {
                total_quality -= max_heap.pop().unwrap();
            }
            if max_heap.len() as i32 == k {
                min_cost = min_cost.min(total_quality as f64 * ratio);
            }
        }
        
        min_cost

    }
}
