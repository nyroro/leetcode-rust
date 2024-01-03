
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let binary = format!("{:b}", n); // 将整数转换为二进制字符串

        let mut max_distance = 0;
        let mut prev_index = -1;
        
        for (i, c) in binary.chars().enumerate() {
            if c == '1' {
                if prev_index != -1 {
                    let distance = i as i32 - prev_index;
                    max_distance = max_distance.max(distance);
                }
                prev_index = i as i32;
            }
        }
        
        max_distance

    }
}
