
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digits = Self::get_digits(n);
        
        for i in 0..31 {
            let power = 1 << i;
            let power_digits = Self::get_digits(power);
            
            if digits == power_digits {
                return true;
            }
        }
        
        false

    }
    
    fn get_digits(n: i32) -> Vec<char> {
        let mut digits: Vec<char> = n.to_string().chars().collect();
        digits.sort();
        digits

    }
}
