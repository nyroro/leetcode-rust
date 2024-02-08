
impl Solution {
    pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
        let n = word.len();
        let mut x: i64 = 0;
        let mut ans = vec![0; n];

        for (i, c) in word.chars().enumerate() {
            x = x * 10 + c.to_digit(10).unwrap() as i64;
            x = x % m as i64;
            if x == 0 {
                ans[i] = 1;
            }
        }

        ans

    }
}
