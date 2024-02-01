
impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let mut ways = 0;
        for pens in 0..=total / cost1 {
            for pencils in 0..=total / cost2 {
                if pens * cost1 + pencils * cost2 == total {
                    ways += 1;
                }
            }
        }
        ways as i64

    }
}
