


impl Solution {
    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut batteries = batteries;
        batteries.sort();
        let mut total = batteries.iter().sum::<i32>() as i64;
        let mut n = n as i32;

        while batteries.last().unwrap() > &(total / n) {
            n -= 1;
            total -= batteries.pop().unwrap() as i64;
        }

        (total / n) as i64

    }
}
