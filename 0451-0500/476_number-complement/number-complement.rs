
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        // 将给定整数转换为二进制表示

        let binary = format!("{:b}", num);
        
        // 遍历二进制表示的每一位，将每个位上的数字取反

        let complement = binary.chars().map(|c| {
            if c == '0' {
                '1'
            } else {
                '0'
            }
        }).collect::<String>();
        
        // 将取反后的二进制表示转换回整数

        let result = i32::from_str_radix(&complement, 2).unwrap();
        
        // 返回转换后的整数作为结果

        result

    }
}
