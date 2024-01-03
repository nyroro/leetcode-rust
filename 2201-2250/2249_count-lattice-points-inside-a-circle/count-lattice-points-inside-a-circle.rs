
use std::collections::HashSet;

impl Solution {
    pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
        let mut points = HashSet::new();
        for circle in circles {
            let xi = circle[0];
            let yi = circle[1];
            let ri = circle[2];
            for x in (xi - ri)..=(xi + ri) {
                for y in (yi - ri)..=(yi + ri) {
                    if (x - xi).pow(2) + (y - yi).pow(2) <= ri.pow(2) {
                        points.insert((x, y));
                    }
                }
            }
        }
        points.len() as i32

    }
}
