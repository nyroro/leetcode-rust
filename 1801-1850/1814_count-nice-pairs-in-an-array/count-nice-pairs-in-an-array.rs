
use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut table: HashMap<i32, i32> = HashMap::new();
        let mut ret = 0;
        let mod_num = 1_000_000_007;

        for &t in nums.iter() {
            let val = t - rev(t);
            *table.entry(val).or_insert(0) += 1;
            ret = (ret + table[&val] - 1) % mod_num;
        }

        ret

    }
}

fn rev(mut x: i32) -> i32 {
    let mut rev = 0;
    while x > 0 {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    rev

}
