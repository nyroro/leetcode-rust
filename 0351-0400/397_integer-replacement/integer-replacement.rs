


impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut count = 0;
        let mut num = n as i64; // Convert n to i64 to avoid overflow when n = 2^31 - 1

        while num != 1 {
            if num % 2 == 0 {
                num /= 2;
            } else if num == 3 || (num as u64 & 2 == 0) {
                num -= 1;
            } else {
                num += 1;
            }
            count += 1;
        }
        count

    }
}
