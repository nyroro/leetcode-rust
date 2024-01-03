
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let times = minutes_to_test / minutes_to_die;
        let mut result = 0;
        while (times + 1).pow(result as u32) < buckets {
            result += 1;
        }
        result

    }
}
