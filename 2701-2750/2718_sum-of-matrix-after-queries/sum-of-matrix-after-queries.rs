
use std::collections::HashMap;

impl Solution {
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
        let mut d: HashMap<i32, (i32, i32)> = HashMap::new();
        let mut ans: i64 = 0;
        let mut cr: i32 = 0;
        let mut cc: i32 = 0;

        for query in queries.iter().rev() {
            let x = query[0];
            let y = query[1];
            let z = query[2];

            if d.get(&y).map_or(true, |v| v.0 == 1 && x == 0 || v.1 == 1 && x == 1) {
                if x == 0 {
                    cr += 1;
                } else {
                    cc += 1;
                }

                let remaining = if x == 0 { n - cc } else { n - cr };
                ans += (remaining * z) as i64;

                let entry = d.entry(y).or_insert((1, 1));
                if x == 0 {
                    entry.0 = 0;
                } else {
                    entry.1 = 0;
                }
            }
        }

        ans

    }
}
