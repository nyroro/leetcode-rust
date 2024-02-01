
impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let mod_num = 1000000007;
        let mut s = arr.iter().sum::<i32>();
        let mut arr_copy = arr.clone();
        
        if k > 1 {
            arr_copy.extend(&arr);
        }
        
        let mut now = 0;
        let mut ret2 = 0;
        
        for &t in &arr_copy {
            now += t;
            if now > 0 {
                ret2 = ret2.max(now);
            } else {
                now = 0;
            }
        }
        
        if k > 2 && s > 0 {
            ret2 = (ret2 + (k - 2) * s) % mod_num;
        }
        
        ret2

    }
}
