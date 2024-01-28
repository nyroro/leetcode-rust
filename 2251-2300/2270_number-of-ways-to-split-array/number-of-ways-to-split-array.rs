
impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let s: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut a: i64 = 0;
        let mut ret: i32 = 0;

        for &t in nums.iter().take(nums.len() - 1) {
            a += t as i64;
            if a >= s - a {
                ret += 1;
            }
        }

        ret

    }
}
