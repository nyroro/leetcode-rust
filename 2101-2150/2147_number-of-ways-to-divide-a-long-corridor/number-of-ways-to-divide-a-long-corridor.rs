
impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut a: Vec<usize> = corridor.chars().enumerate().filter(|&(_, c)| c == 'S').map(|(i, _)| i).collect();
        if a.is_empty() {
            return 0;
        }
        let mut res: i64 = 1;
        for i in (1..a.len() - 1).step_by(2) {
            res *= (a[i + 1] - a[i]) as i64;
            res %= 1_000_000_007;
        }
        if a.len() % 2 == 0 && a.len() >= 2 {
            (res * 1) as i32

        } else {
            0

        }
    }
}
