
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut count = 0;
        let s = n.to_string();
        let digits = s.len();
        
        for i in 0..digits {
            let weight = 10_i32.pow((digits - i - 1) as u32);
            let high = s[..i].parse::<i32>().unwrap_or(0);
            let digit = s[i..=i].parse::<i32>().unwrap_or(0);
            
            if digit > 1 {
                count += high * weight + weight;
            } else if digit == 1 {
                let low = s[i + 1..].parse::<i32>().unwrap_or(0);
                count += high * weight + low + 1;
            }
        }
        
        count

    }
}
