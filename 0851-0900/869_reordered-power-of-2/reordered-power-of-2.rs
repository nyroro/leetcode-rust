
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let digits = Self::get_digits(n);
        let mut power = 1;
        
        while power <= i32::pow(2, 31) {
            let power_digits = Self::get_digits(power);
            
            if digits == power_digits {
                return true;
            }
            
            power *= 2;
        }
        
        false

    }
    
    fn get_digits(n: i32) -> Vec<char> {
        let mut digits: Vec<char> = n.to_string().chars().collect();
        digits.sort();
        digits

    }
}
