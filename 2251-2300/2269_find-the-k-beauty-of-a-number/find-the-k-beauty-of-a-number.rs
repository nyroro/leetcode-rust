
impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let num_str = num.to_string();
        let mut count = 0;
        
        for i in 0..=(num_str.len() - k as usize) {
            let sub = &num_str[i..i + k as usize];
            let sub_num = sub.parse::<i32>().unwrap();
            if sub_num != 0 && num % sub_num == 0 {
                count += 1;
            }
        }
        
        count

    }
}
