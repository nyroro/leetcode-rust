
// 定义一个函数来计算两个数的最大公约数

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a

}

// 定义一个函数来计算两个数的最小公倍数

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

impl Solution {
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut lcm_val = 1;
            for j in i..nums.len() {
                lcm_val = lcm(lcm_val, nums[j]);
                if lcm_val == k {
                    count += 1;
                }
            }
        }
        count

    }
}
