
impl Solution {
    pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        
        for query in queries {
            let x = query[0];
            let y = query[1];
            let r = query[2];
            
            let mut count = 0;
            
            for point in points.iter() {
                let x_i = point[0];
                let y_i = point[1];
                
                let distance = ((x_i - x).pow(2) + (y_i - y).pow(2)) as f64;
                
                if distance <= (r.pow(2) as f64) {
                    count += 1;
                }
            }
            
            result.push(count);
        }
        
        result

    }
}
