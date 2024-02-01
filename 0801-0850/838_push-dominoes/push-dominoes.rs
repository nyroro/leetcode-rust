
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut dominoes: Vec<char> = dominoes.chars().collect();
        let n = dominoes.len();
        let mut forces: Vec<i32> = vec![0; n];

        // 从左向右计算受力

        let mut force = 0;
        for i in 0..n {
            if dominoes[i] == 'R' {
                force = n as i32;
            } else if dominoes[i] == 'L' {
                force = 0;
            } else {
                force = force.max(0) - 1;
            }
            forces[i] += force;
        }

        // 从右向左计算受力

        force = 0;
        for i in (0..n).rev() {
            if dominoes[i] == 'L' {
                force = -n as i32;
            } else if dominoes[i] == 'R' {
                force = 0;
            } else {
                force = force.min(0) + 1;
            }
            forces[i] += force;
        }

        // 根据受力情况更新多米诺骨牌状态

        for i in 0..n {
            if forces[i] > 0 {
                dominoes[i] = 'R';
            } else if forces[i] < 0 {
                dominoes[i] = 'L';
            }
        }

        dominoes.into_iter().collect()
    }
}
