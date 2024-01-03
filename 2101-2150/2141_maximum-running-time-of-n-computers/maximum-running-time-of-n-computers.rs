


impl Solution {
    pub fn max_run_time(n: i32, mut batteries: Vec<i32>) -> i64 {
        batteries.sort();
        let mut total: i64 = batteries.iter().map(|&x| x as i64).sum();
        let mut n = n as i32;

        while *batteries.last().unwrap() as i64 > total / n as i64 {
            n -= 1;
            total -= batteries.pop().unwrap() as i64;
        }

        (total / n as i32 as i64)
    }
}
