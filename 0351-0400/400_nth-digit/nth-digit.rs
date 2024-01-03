
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n as i64;
        let mut len = 1;
        let mut count = 9;
        let mut start = 1;

        while n > len * count {
            n -= len * count;
            len += 1;
            count *= 10;
            start *= 10;
        }

        start += (n - 1) / len;
        let s = start.to_string();
        let result = s.chars().nth(((n - 1) % len) as usize).unwrap();
        result.to_digit(10).unwrap() as i32

    }
}
