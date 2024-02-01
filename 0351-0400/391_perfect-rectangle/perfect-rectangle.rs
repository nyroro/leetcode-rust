
use std::collections::HashSet;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut points = HashSet::new();
        let mut area = 0;

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
                    points.insert(p);
                }
            }
        }

        if points.len() != 4 {
            return false;
        }

        let mut corner = HashSet::new();
        corner.insert((rectangles[0][0], rectangles[0][1]));
        corner.insert((rectangles[0][0], rectangles[0][3]));
        corner.insert((rectangles[0][2], rectangles[0][1]));
        corner.insert((rectangles[0][2], rectangles[0][3]));

        for p in &points {
            if !corner.contains(p) {
                return false;
            }
        }

        area == (corner.iter().max_by_key(|&&(x, y)| x * y).unwrap().0 - corner.iter().min_by_key(|&&(x, y)| x * y).unwrap().0) * (corner.iter().max_by_key(|&&(x, y)| x * y).unwrap().1 - corner.iter().min_by_key(|&&(x, y)| x * y).unwrap().1)
    }
}
