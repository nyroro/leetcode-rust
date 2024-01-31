
impl Solution {
    pub fn minimize_set(divisor1: i32, divisor2: i32, unique_cnt1: i32, unique_cnt2: i32) -> i32 {
        // Define a helper function to calculate the greatest common divisor

        fn gcd(a: i64, b: i64) -> i64 {
            if b == 0 {
                a

            } else {
                gcd(b, a % b)
            }
        }

        // Calculate the least common multiple

        fn lcm(a: i64, b: i64) -> i64 {
            a * b / gcd(a, b)
        }

        // Initialize variables with input parameters

        let (d1, d2, u1, u2) = (divisor1 as i64, divisor2 as i64, unique_cnt1 as i64, unique_cnt2 as i64);
        let (mut l, mut r) = (1i64, 10000000000i64);
        let mut res = i64::MAX;

        // Perform binary search to find the minimum possible maximum integer

        while l <= r {
            let m = (l + r) / 2;
            let x = m - m / d1;
            let y = m - m / d2;
            let z = m - m / lcm(d1, d2);

            if x < u1 || y < u2 || z < u1 + u2 {
                l = m + 1;
            } else {
                res = res.min(m);
                r = m - 1;
            }
        }

        res as i32

    }
}
