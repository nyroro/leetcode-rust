
impl Solution {
    pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
        let pens_max = total / cost1;
        let pencils_max = total / cost2;
        
        let mut ways = 0;
        for pens in 0..=pens_max {
            let pencils = (total - pens * cost1) / cost2;
            ways += (pencils + 1) as i64;
        }
        
        ways

    }
}
