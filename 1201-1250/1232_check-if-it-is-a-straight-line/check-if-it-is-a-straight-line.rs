
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let n = coordinates.len();
        if n == 2 {
            return true;
        }
        let x0 = coordinates[0][0];
        let y0 = coordinates[0][1];
        let x1 = coordinates[1][0];
        let y1 = coordinates[1][1];
        for i in 2..n {
            let xi = coordinates[i][0];
            let yi = coordinates[i][1];
            if (x1 - x0) * (yi - y0) != (xi - x0) * (y1 - y0) {
                return false;
            }
        }
        true

    }
}
