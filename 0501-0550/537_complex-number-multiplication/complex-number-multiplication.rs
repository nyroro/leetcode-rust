
impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        // 解析num1和num2的实部和虚部

        let num1_parts: Vec<&str> = num1.split('+').collect();
        let num2_parts: Vec<&str> = num2.split('+').collect();
        
        let num1_real: i32 = num1_parts[0].parse().unwrap();
        let num1_imaginary: i32 = num1_parts[1].trim_end_matches('i').parse().unwrap();
        
        let num2_real: i32 = num2_parts[0].parse().unwrap();
        let num2_imaginary: i32 = num2_parts[1].trim_end_matches('i').parse().unwrap();
        
        // 计算实部和虚部的乘积

        let real = num1_real * num2_real - num1_imaginary * num2_imaginary;
        let imaginary = num1_real * num2_imaginary + num1_imaginary * num2_real;
        
        // 构造结果字符串

        format!("{}+{}i", real, imaginary)
    }
}
