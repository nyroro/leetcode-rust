


impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut r = vec![0; n];

        for i in (0..n).rev() {
            let b = questions[i][1] as usize;
            let p = questions[i][0] as i64;
            if i == n - 1 {
                r[i] = p;
            } else if i + b + 1 >= n {
                r[i] = r[i + 1];
                if p > r[i] {
                    r[i] = p;
                }
            } else {
                r[i] = r[i + 1];
                if r[i] < p + r[i + b + 1] {
                    r[i] = p + r[i + b + 1];
                }
            }
        }

        r[0]
    }
}
