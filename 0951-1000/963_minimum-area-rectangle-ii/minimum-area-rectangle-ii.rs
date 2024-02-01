
impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        use std::collections::HashSet;
        
        let mut min_area = std::f64::MAX;
        let point_set: HashSet<(i32, i32)> = points.into_iter().map(|p| (p[0], p[1])).collect();
        
        for i in 0..point_set.len() {
            let p1 = point_set.iter().nth(i).unwrap();
            for j in (i + 1)..point_set.len() {
                let p2 = point_set.iter().nth(j).unwrap();
                for k in (j + 1)..point_set.len() {
                    let p3 = point_set.iter().nth(k).unwrap();
                    let x1 = p2.0 - p1.0;
                    let y1 = p2.1 - p1.1;
                    let x2 = p3.0 - p1.0;
                    let y2 = p3.1 - p1.1;
                    
                    if x1 * x2 + y1 * y2 == 0 {
                        let p4 = (p3.0 + p2.0 - p1.0, p3.1 + p2.1 - p1.1);
                        if point_set.contains(&p4) {
                            let area = (x1 * x1 + y1 * y1).sqrt() * (x2 * x2 + y2 * y2).sqrt();
                            min_area = min_area.min(area);
                        }
                    }
                }
            }
        }
        
        if min_area == std::f64::MAX {
            return 0.0;
        }
        
        min_area

    }
}
