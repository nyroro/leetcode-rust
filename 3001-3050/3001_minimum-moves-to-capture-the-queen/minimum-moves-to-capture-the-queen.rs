
impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        // same row

        if e == a {
            if c != a { // bishop not in the same row

                return 1;
            }
            // check bishop column not in between

            if !(d > b && d < f) && !(d < b && d > f) {
                return 1;
            }
        }
        
        // same column

        if f == b {
            if d != b { // bishop not in the same column

                return 1;
            }
            // check bishop row not in between

            if !(c > a && c < e) && !(c < a && c > e) {
                return 1;
            }
        }
        
        // same diagonal 1

        if e - f == c - d {
            if a - b != c - d { // rook not in the same diagonal

                return 1;
            }
            // checking if the row of the rook is not in between

            if !(a > c && a < e) && !(a < c && a > e) {
                return 1;
            }
        }
        
        // same diagonal 2

        if e + f == c + d {
            if a + b != c + d { // rook not in the same diagonal

                return 1;
            }
            // checking if the row of the rook is not in between

            if !(a > c && a < e) && !(a < c && a > e) {
                return 1;
            }
        }
        
        return 2;
    }
}
