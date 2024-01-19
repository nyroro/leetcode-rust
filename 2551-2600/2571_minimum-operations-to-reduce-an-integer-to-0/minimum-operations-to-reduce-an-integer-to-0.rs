
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut num = n;
        let mut count = 0;
        
        while num > 0 {
            if num & 1 == 0 {
                num >>= 1;
            } else if num == 1 {
                count += 1;
                break;
            } else if num & 2 == 0 {
                num -= 1;
                count += 1;
            } else {
                num += 1;
                count += 1;
            }
        }
        
        count

    }
}
