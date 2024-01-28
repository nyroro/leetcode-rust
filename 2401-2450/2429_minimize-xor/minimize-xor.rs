
impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut c = 0;
        for i in 0..32 {
            if num2 & (1 << i) != 0 {
                c += 1;
            }
        }
        let mut num = 0;
        for i in (0..32).rev() {
            if (num1 & (1 << i)) != 0 && c > 0 {
                num += 2_i32.pow(i as u32);
                c -= 1;
            }
        }
        let mut s = format!("{:032b}", num);
        let mut i = 31;
        while c > 0 && i > 0 {
            if s.chars().nth(i).unwrap() == '0' {
                s = s.chars().take(i).collect::<String>() + "1" + &s.chars().skip(i + 1).collect::<String>();
                c -= 1;
            } else {
                i -= 1;
            }
        }
        i32::from_str_radix(&s, 2).unwrap()
    }
}
