
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut count0 = 0;
        let mut count1 = 0;

        for i in 0..n {
            let c = s[i];
            if c == '0' {
                if i % 2 == 0 {
                    count1 += 1;
                } else {
                    count0 += 1;
                }
            } else {
                if i % 2 == 0 {
                    count0 += 1;
                } else {
                    count1 += 1;
                }
            }
        }

        if n % 2 == 0 {
            return std::cmp::min(count0, count1);
        }

        let mut dp_forward = vec![[0; 2]; n];
        let mut dp_backward = vec![[0; 2]; n];

        if s[0] == '0' {
            dp_forward[0][1] = 1;
        } else {
            dp_forward[0][0] = 1;
        }

        for i in 1..n {
            if s[i] == '0' {
                dp_forward[i][0] = dp_forward[i - 1][1];
                dp_forward[i][1] = dp_forward[i - 1][0] + 1;
            } else {
                dp_forward[i][0] = dp_forward[i - 1][1] + 1;
                dp_forward[i][1] = dp_forward[i - 1][0];
            }
        }

        if s[n - 1] == '0' {
            dp_backward[n - 1][1] = 1;
        } else {
            dp_backward[n - 1][0] = 1;
        }

        for i in (0..n - 1).rev() {
            if s[i] == '0' {
                dp_backward[i][0] = dp_backward[i + 1][1];
                dp_backward[i][1] = dp_backward[i + 1][0] + 1;
            } else {
                dp_backward[i][0] = dp_backward[i + 1][1] + 1;
                dp_backward[i][1] = dp_backward[i + 1][0];
            }
        }

        let mut min_count = std::cmp::min(count0, count1);

        for i in 1..n {
            let cur_min = std::cmp::min(
                dp_forward[i - 1][0] + dp_backward[i][0],
                dp_forward[i - 1][1] + dp_backward[i][1],
            );
            min_count = std::cmp::min(min_count, cur_min);
        }

        min_count

    }
}
