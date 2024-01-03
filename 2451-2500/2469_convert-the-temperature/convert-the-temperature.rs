
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        // 将摄氏度转换为开尔文

        let kelvin = celsius + 273.15;
        
        // 将摄氏度转换为华氏度

        let fahrenheit = celsius * 1.80 + 32.00;
        
        // 将转换后的开尔文和华氏度存储在数组中并返回

        vec![kelvin, fahrenheit]
    }
}
