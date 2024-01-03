
use std::collections::HashMap;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        if n < 3 {
            return n as i32;
        }
        
        let mut max_points = 0;
        
        for i in 0..n {
            let mut slope_count: HashMap<(i32, i32), i32> = HashMap::new();
            let mut duplicate = 0;
            let mut curr_max = 0;
            
            for j in (i + 1)..n {
                let x1 = points[i][0];
                let y1 = points[i][1];
                let x2 = points[j][0];
                let y2 = points[j][1];
                
                if x1 == x2 && y1 == y2 {
                    duplicate += 1;
                    continue;
                }
                
                let dx = x2 - x1;
                let dy = y2 - y1;
                let gcd = Solution::gcd(dx, dy);
                let slope = (dx / gcd, dy / gcd);
                
                let count = slope_count.entry(slope).or_insert(0);
                *count += 1;
                curr_max = curr_max.max(*count);
            }
            
            max_points = max_points.max(curr_max + duplicate + 1);
        }
        
        max_points

    }
    
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a

        } else {
            Solution::gcd(b, a % b)
        }
    }
}
