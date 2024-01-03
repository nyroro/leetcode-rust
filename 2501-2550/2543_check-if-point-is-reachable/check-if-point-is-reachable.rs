


impl Solution {
    pub fn is_reachable(target_x: i32, target_y: i32) -> bool {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a

            } else {
                gcd(b, a % b)
            }
        }

        let x = gcd(target_x, target_y);
        x & (x - 1) == 0

    }
}
