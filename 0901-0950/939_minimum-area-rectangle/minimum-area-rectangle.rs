
use std::collections::HashSet;

impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let mut min_area = std::i32::MAX;
        let point_set: HashSet<Vec<i32>> = points.iter().cloned().collect();

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let p1 = &points[i];
                let p2 = &points[j];

                if p1[0] != p2[0] && p1[1] != p2[1] {
                    if point_set.contains(&vec![p1[0], p2[1]]) && point_set.contains(&vec![p2[0], p1[1]]) {
                        let area = (p2[0] - p1[0]).abs() * (p2[1] - p1[1]).abs();
                        min_area = min_area.min(area);
                    }
                }
            }
        }

        if min_area == std::i32::MAX {
            return 0;
        }

        min_area

    }
}
