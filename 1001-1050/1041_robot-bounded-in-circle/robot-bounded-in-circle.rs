
impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut direction = 0; // 0表示北，1表示东，2表示南，3表示西


        for ch in instructions.chars() {
            match ch {
                'G' => {
                    match direction {
                        0 => y += 1,
                        1 => x += 1,
                        2 => y -= 1,
                        3 => x -= 1,
                        _ => (),
                    }
                },
                'L' => direction = (direction + 3) % 4,
                'R' => direction = (direction + 1) % 4,
                _ => (),
            }
        }

        (x == 0 && y == 0) || direction != 0

    }
}
