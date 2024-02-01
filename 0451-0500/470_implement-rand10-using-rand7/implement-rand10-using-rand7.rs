
/** 
 * The rand7() API is already defined for you.
 * @return a random integer in the range 1 to 7

 * fn rand7() -> i32;
 */

impl Solution {
    pub fn rand10() -> i32 {
        let mut num = 0;
        while num > 40 {
            let a = rand7();
            let b = rand7();
            num = (a - 1) * 7 + b;
        }
        (num % 10) + 1

    }
}
