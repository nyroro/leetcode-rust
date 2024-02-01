
use std::collections::HashMap;

struct DetectSquares {
    points: HashMap<(i32, i32), i32>,
}

impl DetectSquares {
    fn new() -> Self {
        DetectSquares {
            points: HashMap::new(),
        }
    }
    
    fn add(&mut self, point: Vec<i32>) {
        let x = point[0];
        let y = point[1];
        *self.points.entry((x, y)).or_insert(0) += 1;
    }
    
    fn count(&self, point: Vec<i32>) -> i32 {
        let x = point[0];
        let y = point[1];
        let mut count = 0;
        
        for (&(px, py), &c) in &self.points {
            if px != x && py != y && px - x == py - y {
                let pair_count = self.points.get(&(px, y)).unwrap_or(&0) * self.points.get(&(x, py)).unwrap_or(&0);
                count += pair_count * c;
            }
        }
        
        count

    }
}
