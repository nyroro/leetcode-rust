
impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut direction = 0;
        let mut max_distance = 0;
        let mut obstacles_set = std::collections::HashSet::new();
        
        for obstacle in obstacles {
            obstacles_set.insert((obstacle[0], obstacle[1]));
        }
        
        let dx = vec![0, 1, 0, -1];
        let dy = vec![1, 0, -1, 0];
        
        for command in commands {
            if command == -2 {
                direction = (direction + 3) % 4;
            } else if command == -1 {
                direction = (direction + 1) % 4;
            } else {
                for _ in 0..command {
                    let next_x = x + dx[direction as usize];
                    let next_y = y + dy[direction as usize];
                    
                    if obstacles_set.contains(&(next_x, next_y)) {
                        break;
                    }
                    
                    x = next_x;
                    y = next_y;
                    max_distance = max_distance.max(x * x + y * y);
                }
            }
        }
        
        max_distance

    }
}
