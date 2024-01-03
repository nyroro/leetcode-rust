
impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        // 计算箱子的体积

        let volume = (length as i64) * (width as i64) * (height as i64);
        
        // 判断箱子的类别

        if length >= 10000 || width >= 10000 || height >= 10000 || volume >= 1000000000 {
            if mass >= 100 {
                return String::from("Both");
            } else {
                return String::from("Bulky");
            }
        } else if mass >= 100 {
            return String::from("Heavy");
        } else {
            return String::from("Neither");
        }
    }
}
