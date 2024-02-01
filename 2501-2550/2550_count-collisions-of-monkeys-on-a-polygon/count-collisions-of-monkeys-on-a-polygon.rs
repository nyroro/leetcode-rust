
impl Solution {
    pub fn monkey_move(n: i32) -> i32 {
        let modulo = 1_000_000_007;
        let mut answer: i64 = 1;
        let mut exponent: i64 = 2;
        let mut num = n as i64;

        while num > 0 {
            if num & 1 == 1 {
                answer = (answer * exponent) % modulo;
            }
            exponent = (exponent * exponent) % modulo;
            num >>= 1;
        }

        answer = (answer + modulo - 2) % modulo;
        answer as i32

    }
}
