
impl Solution {
    // 计算一个整数的二进制表示中包含的 set bits 的数量

    fn count_set_bits(mut n: i32) -> i32 {
        let mut count = 0;
        while n > 0 {
            count += n & 1;
            n >>= 1;
        }
        count

    }

    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        for (i, &num) in nums.iter().enumerate() {
            if Solution::count_set_bits(i as i32) == k {
                sum += num;
            }
        }
        sum

    }
}
