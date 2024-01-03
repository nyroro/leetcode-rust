
use std::collections::HashSet;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut points = HashSet::new();
        let mut area = 0;

        let mut corner = HashSet::new();

        for rect in &rectangles {
            let (x1, y1, x2, y2) = (rect[0], rect[1], rect[2], rect[3]);
            area += (x2 - x1) * (y2 - y1);

            let p1 = (x1, y1);
            let p2 = (x1, y2);
            let p3 = (x2, y1);
            let p4 = (x2, y2);

            for p in &[p1, p2, p3, p4] {
                if points.contains(p) {
                    points.remove(p);
                } else {
                    points.insert(*p);
                }
            }

            if corner.contains(&p1) {
                corner.remove(&p1);
            } else {
                corner.insert(p1);
            }
            if corner.contains(&p2) {
                corner.remove(&p2);
            } else {
                corner.insert(p2);
            }
            if corner.contains(&p3) {
                corner.remove(&p3);
            } else {
                corner.insert(p3);
            }
            if corner.contains(&p4) {
                corner.remove(&p4);
            } else {
                corner.insert(p4);
            }
        }

        if points.len() != 4 || corner.len() != 4 {
            return false;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for p in &corner {
            min_x = min_x.min(p.0);
            min_y = min_y.min(p.1);
            max_x = max_x.max(p.0);
            max_y = max_y.max(p.1);
        }

        area == (max_x - min_x) * (max_y - min_y)
    }
}
