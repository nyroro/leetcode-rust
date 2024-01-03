
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        // 使用欧几里得算法计算最大公约数

        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 {
                a

            } else {
                gcd(b, a % b)
            }
        }
        
        // 计算最小公倍数

        fn lcm(a: i32, b: i32) -> i32 {
            a * b / gcd(a, b)
        }
        
        // 返回最小公倍数

        lcm(n, 2)
    }
}
