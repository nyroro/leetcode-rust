
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = dist.len();
        let mut arrival_time: Vec<f64> = Vec::new();
        
        // Calculate arrival time for each monster

        for i in 0..n {
            let time = (dist[i] as f64) / (speed[i] as f64);
            arrival_time.push(time);
        }
        
        // Sort arrival time in ascending order

        arrival_time.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let mut eliminated = 0;
        let mut current_time = 0.0;
        
        // Eliminate monsters based on arrival time

        for &time in &arrival_time {
            if time > current_time {
                current_time += 1.0;
                eliminated += 1;
            } else {
                break;
            }
        }
        
        eliminated

    }
}
