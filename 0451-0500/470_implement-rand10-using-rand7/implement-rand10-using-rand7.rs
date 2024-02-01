
impl Solution {
    pub fn rand10() -> i32 {
        let mut num = 0;
        while num == 0 {
            let a = rand7();
            let b = rand7();
            num = (a - 1) * 7 + b;
            if num > 40 {
                num = 0;
            }
        }
        (num % 10) + 1

    }
}
