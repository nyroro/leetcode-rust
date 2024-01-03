
impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let mut current = 1;
        let mut direction = 1;

        for _ in 0..time {
            if direction == 1 {
                current += 1;
            } else {
                current -= 1;
            }

            if current == 1 {
                direction = 1;
            } else if current == n {
                direction = -1;
            }
        }

        current

    }
}
