
impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let mut l = vec![0; security.len()];
        let mut r = vec![0; security.len()];
        
        for i in 1..security.len() {
            if security[i] <= security[i - 1] {
                l[i] = l[i - 1] + 1;
            }
        }
        
        for i in (0..security.len() - 1).rev() {
            if security[i] <= security[i + 1] {
                r[i] = r[i + 1] + 1;
            }
        }
        
        let mut ret = Vec::new();
        for i in 0..security.len() {
            if l[i] >= time && r[i] >= time {
                ret.push(i as i32);
            }
        }
        
        ret

    }
}
