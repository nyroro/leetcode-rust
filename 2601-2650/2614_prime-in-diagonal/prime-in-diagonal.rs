
impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        // 判断一个数是否为素数

        fn is_prime(num: i32) -> bool {
            if num <= 1 {
                return false;
            }
            for i in 2..=(num as f64).sqrt() as i32 {
                if num % i == 0 {
                    return false;
                }
            }
            true

        }
        
        let mut max_prime = 0;
        let n = nums.len();
        
        for i in 0..n {
            if is_prime(nums[i][i]) {
                max_prime = max_prime.max(nums[i][i]);
            }
            if is_prime(nums[i][n - i - 1]) {
                max_prime = max_prime.max(nums[i][n - i - 1]);
            }
        }
        
        max_prime

    }
}
