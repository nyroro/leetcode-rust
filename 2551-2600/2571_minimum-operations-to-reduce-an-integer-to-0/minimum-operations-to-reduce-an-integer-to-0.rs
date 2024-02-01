
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut count = 0;
        let mut num = n;
        
        while num > 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            count += 1;
        }
        
        count

    }
}
