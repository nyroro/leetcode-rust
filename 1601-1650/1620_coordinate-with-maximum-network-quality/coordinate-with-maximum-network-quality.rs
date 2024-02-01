
impl Solution {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        let mut max_quality = 0;
        let mut result = vec![0, 0];
        
        for x in 0..=50 {
            for y in 0..=50 {
                let mut total_quality = 0;
                
                for tower in &towers {
                    let xi = tower[0];
                    let yi = tower[1];
                    let qi = tower[2];
                    
                    let distance = ((x - xi).pow(2) + (y - yi).pow(2)) as f64;
                    
                    if distance <= radius.pow(2) {
                        let signal_quality = (qi as f64 / (1.0 + distance.sqrt())).floor() as i32;
                        total_quality += signal_quality;
                    }
                }
                
                if total_quality > max_quality || (total_quality == max_quality && (x < result[0] || (x == result[0] && y < result[1]))) {
                    max_quality = total_quality;
                    result = vec![x, y];
                }
            }
        }
        
        result

    }
}
