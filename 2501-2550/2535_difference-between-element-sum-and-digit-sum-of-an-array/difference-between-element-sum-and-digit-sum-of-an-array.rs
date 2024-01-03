
impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        // 计算数组中所有元素的和

        let element_sum: i32 = nums.iter().sum();
        
        // 计算数组中所有数字的各个位数之和

        let mut digit_sum: i32 = 0;
        for num in nums {
            let mut n = num;
            while n > 0 {
                digit_sum += n % 10;
                n /= 10;
            }
        }
        
        // 计算绝对差值

        let abs_diff = (element_sum - digit_sum).abs();
        
        // 返回绝对差值作为结果

        abs_diff

    }
}
