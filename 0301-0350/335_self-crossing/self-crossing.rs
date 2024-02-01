
use std::collections::HashSet;

impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        
        let mut x = 0;
        let mut y = 0;
        let mut direction = 0;
        
        for d in distance {
            if direction == 0 {
                for _ in 0..d {
                    y += 1;
                    if !visited.insert((x, y)) {
                        return true;
                    }
                }
                direction = 1;
            } else if direction == 1 {
                for _ in 0..d {
                    x -= 1;
                    if !visited.insert((x, y)) {
                        return true;
                    }
                }
                direction = 2;
            } else if direction == 2 {
                for _ in 0..d {
                    y -= 1;
                    if !visited.insert((x, y)) {
                        return true;
                    }
                }
                direction = 3;
            } else {
                for _ in 0..d {
                    x += 1;
                    if !visited.insert((x, y)) {
                        return true;
                    }
                }
                direction = 0;
            }
        }
        
        false

    }
}
