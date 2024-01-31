


impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let s = moves.chars().filter(|&c| c == 'L').count();
        let t = moves.chars().filter(|&c| c == 'R').count();

        let mut distance: i32 = 0;

        for c in moves.chars() {
            match c {
                'L' => distance -= 1,
                'R' => distance += 1,
                _ => {
                    if s > t {
                        distance -= 1;
                    } else if t > s {
                        distance += 1;
                    } else {
                        distance += 1;
                    }
                }
            }
        }

        if distance < 0 {
            -distance

        } else {
            distance

        }
    }
}
