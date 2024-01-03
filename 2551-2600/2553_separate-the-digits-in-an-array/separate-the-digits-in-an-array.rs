
impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::new();  // 创建一个新的数组来存储结果

        for num in nums {
            let mut temp = num;  // 临时变量存储当前整数

            let mut digits = Vec::new();  // 创建一个新的数组来存储当前整数的各个数字

            while temp > 0 {
                digits.push(temp % 10);  // 将当前整数的个位数字添加到数组中

                temp /= 10;  // 去掉当前整数的个位数字

            }
            digits.reverse();  // 将数组中的数字反转，以保持它们在原整数中的顺序

            answer.append(&mut digits);  // 将当前整数的各个数字添加到结果数组中

        }
        answer  // 返回结果数组

    }
}
