
struct Bomb {
    x: i32,
    y: i32,
    r: i32,
}

impl Bomb {
    fn distance(&self, other: &Bomb) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut max_detonated = 0;
        let n = bombs.len();

        for i in 0..n {
            let mut visited = vec![false; n];
            let mut count = 1;
            let mut queue = vec![Bomb { x: bombs[i][0], y: bombs[i][1], r: bombs[i][2] }];
            visited[i] = true;

            while let Some(b) = queue.pop() {
                for j in 0..n {
                    if !visited[j] {
                        let dist = b.distance(&Bomb { x: bombs[j][0], y: bombs[j][1], r: bombs[j][2] });
                        if dist <= b.r as f64 {
                            count += 1;
                            queue.push(Bomb { x: bombs[j][0], y: bombs[j][1], r: bombs[j][2] });
                            visited[j] = true;
                        }
                    }
                }
            }

            max_detonated = max_detonated.max(count);
        }

        max_detonated

    }
}
