


impl Solution {
    pub fn get_probability(balls: Vec<i32>) -> f64 {
        let mut match_count = 0.0;
        let mut total_count = 0.0;
        let mut sum = 0.0;

        let n = balls.len();
        sum = balls.iter().sum::<i32>() as f64 / 2.0;

        // Pre-calculate combinations

        let mut comb: Vec<Vec<f64>> = vec![vec![0.0; 11]; 11];
        for i in 0..11 {
            comb[i][i] = 1.0;
            comb[i][0] = 1.0;
            for j in 1..i {
                comb[i][j] = comb[i - 1][j - 1] + comb[i - 1][j];
            }
        }

        // DFS function to calculate probability

        fn dfs(
            pos: usize,
            combined: f64,
            ldistinct: f64,
            rdistinct: f64,
            perm: f64,
            balls: &Vec<i32>,
            comb: &Vec<Vec<f64>>,
            match_count: &mut f64,
            total_count: &mut f64,
            sum: f64,
        ) {
            if pos == balls.len() {
                if combined == sum {
                    if ldistinct == rdistinct {
                        *match_count += perm;
                    }
                    *total_count += perm;
                }
                return;
            }
            for i in 0..=balls[pos] {
                if i as f64 + combined <= sum {
                    let ld = ldistinct + if i > 0 { 1.0 } else { 0.0 };
                    let rd = rdistinct + if balls[pos] - i > 0 { 1.0 } else { 0.0 };
                    dfs(
                        pos + 1,
                        i as f64 + combined,
                        ld,
                        rd,
                        perm * comb[balls[pos] as usize][i as usize],
                        balls,
                        comb,
                        match_count,
                        total_count,
                        sum,
                    );
                }
            }
        }

        dfs(0, 0.0, 0.0, 0.0, 1.0, &balls, &comb, &mut match_count, &mut total_count, sum);
        match_count / total_count

    }
}
