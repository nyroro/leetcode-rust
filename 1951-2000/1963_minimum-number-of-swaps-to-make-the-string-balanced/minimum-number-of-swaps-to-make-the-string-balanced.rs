
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut swaps = 0;
        let mut count = 0;

        for c in s.chars() {
            if c == '[' {
                count += 1;
            } else {
                count -= 1;
                if count < 0 {
                    swaps += 1;
                    count = 1;  // Fix: Reset count to 1 instead of 0

                }
            }
        }

        swaps

    }
}
