
use std::collections::HashSet;



impl Solution {
    pub fn score_of_students(s: String, answers: Vec<i32>) -> i32 {
        let n = s.len();
        let mut dp: Vec<Vec<HashSet<i32>>> = vec![vec![HashSet::new(); n]; n];

        fn evaluate(s: &Vec<char>, i: usize, j: usize, dp: &mut Vec<Vec<HashSet<i32>>>) -> HashSet<i32> {
            if !dp[i][j].is_empty() {
                return dp[i][j].clone();
            }

            let mut result = HashSet::new();
            if i == j {
                result.insert(s[i].to_digit(10).unwrap() as i32);
            } else {
                for k in (i + 1..j).step_by(2) {
                    let left = evaluate(s, i, k - 1, dp);
                    let right = evaluate(s, k + 1, j, dp);
                    for &x in &left {
                        for &y in &right {
                            let val = if s[k] == '+' { x + y } else { x * y };
                            if val <= 1000 {
                                result.insert(val);
                            }
                        }
                    }
                }
            }
            dp[i][j] = result.clone();
            result

        }

        let s: Vec<char> = s.chars().collect();
        let mut v: Vec<i32> = Vec::new();
        for ch in &s {
            if v.last().map_or(false, |&last| last == -1) {
                v.pop();
                let num = v.pop().unwrap() * (*ch as i32 - '0' as i32);
                v.push(num);
            } else if *ch == '*' {
                v.push(-1);
            } else if *ch == '+' {
                v.push(-2);
            } else {
                v.push((*ch as i32 - '0' as i32));
            }
        }

        let mut right = 0;
        for i in (0..v.len()).step_by(2) {
            right += v[i];
        }

        let correct_result = evaluate(&s, 0, n - 1, &mut dp);

        let mut points = 0;
        for &a in &answers {
            if a == right {
                points += 5;
            } else if correct_result.contains(&a) {
                points += 2;
            }
        }

        points

    }
}
