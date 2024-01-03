
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();
        
        for asteroid in asteroids {
            if asteroid > 0 {
                stack.push(asteroid);
            } else {
                let mut destroyed = false;
                while let Some(&top) = stack.last() {
                    if top < 0 {
                        stack.push(asteroid);
                        destroyed = true;
                        break;
                    } else if top == -asteroid {
                        stack.pop();
                        destroyed = true;
                        break;
                    } else if top > -asteroid {
                        destroyed = true;
                        break;
                    } else {
                        stack.pop();
                        if stack.is_empty() {
                            stack.push(asteroid);
                            destroyed = true;
                            break;
                        }
                    }
                }
                if !destroyed {
                    stack.push(asteroid);
                }
            }
        }
        
        stack

    }
}
