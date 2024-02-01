
impl Solution {
    pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for x in 1..=100 {
            for y in 1..=100 {
                let mut inside_circle = false;
                for circle in &circles {
                    let xi = circle[0];
                    let yi = circle[1];
                    let ri = circle[2];
                    if (x - xi).pow(2) + (y - yi).pow(2) <= ri.pow(2) {
                        inside_circle = true;
                        break;
                    }
                }
                if inside_circle {
                    count += 1;
                }
            }
        }
        count

    }
}
