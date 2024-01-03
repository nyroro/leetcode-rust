
impl Solution {
    pub fn min_flips(target: String) -> i32 {
        let mut flips = 0;
        let mut prev = '0';

        for ch in target.chars() {
            if ch != prev {
                flips += 1;
                prev = ch;
            }
        }

        flips

    }
}
