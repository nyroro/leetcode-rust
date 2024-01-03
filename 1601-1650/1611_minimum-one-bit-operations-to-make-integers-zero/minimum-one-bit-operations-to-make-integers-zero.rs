
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        // 定义一个内联函数来找到最右边的1

        fn lowbit(x: i32) -> i32 {
            x & -x

        }
        
        // 如果n小于等于1，返回n

        if n <= 1 {
            return n;
        }
        
        // 定义一个变量first，初始化为全1

        let mut first = !0;
        // 当first与n的按位与结果不为0时，将first左移一位

        while first & n != 0 {
            first <<= 1;
        }
        // 将first右移一位，并找到第二个bit

        first = lowbit(first >> 1);
        let second = (first >> 1) & n;
        // 递归调用minimum_one_bit_operations函数

        let rest = Solution::minimum_one_bit_operations(n & !first & !second);
        // 如果second不为0，返回first + rest，否则返回(first << 1) - 1 - rest

        if second != 0 {
            first + rest

        } else {
            (first << 1) - 1 - rest

        }
    }
}
