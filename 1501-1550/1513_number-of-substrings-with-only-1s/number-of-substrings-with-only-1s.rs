
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut count = 0;
        let mut result = 0;
        let modulo = 1000000007;

        for c in s.chars() {
            if c == '1' {
                count += 1;
            } else {
                count = 0;
            }
            result = (result + count) % modulo;
        }

        result

    }
}
