
impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        let reversed1 = Solution::reverse_integer(num);
        let reversed2 = Solution::reverse_integer(reversed1);
        return reversed2 == num;
    }

    fn reverse_integer(x: i32) -> i32 {
        let mut num = x;
        let mut reversed = 0;

        while num != 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }

        reversed

    }
}
