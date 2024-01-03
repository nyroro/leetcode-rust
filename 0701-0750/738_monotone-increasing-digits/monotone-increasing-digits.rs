
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits = n.to_string().chars().collect::<Vec<char>>();
        let mut i = digits.len() as i32 - 1;
        
        while i > 0 {
            let curr = digits[i as usize];
            let prev = digits[(i - 1) as usize];
            if curr < prev {
                digits[(i - 1) as usize] = (prev.to_digit(10).unwrap() - 1).to_string().chars().next().unwrap();
                for j in (i as usize)..digits.len() {
                    digits[j] = '9';
                }
            }
            i -= 1;
        }
        
        digits.iter().collect::<String>().parse::<i32>().unwrap()
    }
}
