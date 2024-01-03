
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut rev: i32 = 0;
        let mut num: i32 = x;
        
        while num != 0 {
            let digit = num % 10;
            num /= 10;
            
            // 检查加法操作是否会溢出

            if rev.checked_mul(10).is_none() || rev.checked_add(digit).is_none() {
                return 0;
            }
            
            rev = rev * 10 + digit;
        }
        
        rev

    }
}
