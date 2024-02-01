
impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let mut image = image;
        let start_color = image[sr as usize][sc as usize];
        if start_color == color {
            return image;
        }
        
        let m = image.len();
        let n = image[0].len();
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        
        let mut queue = VecDeque::new();
        queue.push_back((sr as usize, sc as usize));
        
        while let Some((r, c)) = queue.pop_front() {
            if image[r][c] == start_color {
                image[r][c] = color;
                for (dr, dc) in &directions {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < m as i32 && nc >= 0 && nc < n as i32 {
                        queue.push_back((nr as usize, nc as usize));
                    }
                }
            }
        }
        
        image

    }
}
