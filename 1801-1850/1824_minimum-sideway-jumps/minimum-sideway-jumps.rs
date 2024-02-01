
impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len();
        let mut f = vec![1, 0, 1];
        let inf = 1 << 30;

        for i in 1..n {
            for j in 0..3 {
                if obstacles[i] == j as i32 + 1 {
                    f[j] = inf;
                    break;
                }
            }
            let x = f.iter().min().unwrap() + 1;
            for j in 0..3 {
                if obstacles[i] != j as i32 + 1 {
                    f[j] = f[j].min(x);
                }
            }
        }

        f.iter().min().unwrap().clone()
    }
}
