
impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        let mut final_sum = final_sum;
        if final_sum % 2 != 0 {
            return vec![];
        }
        
        final_sum /= 2;
        
        let mut now = 1;
        let mut ret = Vec::new();
        while final_sum > 2 * now {
            ret.push(now * 2);
            final_sum -= now;
            now += 1;
        }
        ret.push(final_sum * 2);
        ret

    }
}
