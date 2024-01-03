
impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let first_digit = nums[i] / 10_i32.pow((nums[i] as f64).log10() as u32);
            let last_digit = nums[i] % 10;
            for j in (i + 1)..nums.len() {
                let gcd = Self::gcd(first_digit, nums[j] % 10);
                if gcd == 1 {
                    count += 1;
                }
            }
        }
        count

    }
    
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a

        } else {
            Self::gcd(b, a % b)
        }
    }
}
