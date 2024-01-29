


impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut even_list: Vec<i32> = Vec::new();
        let mut odd_list: Vec<i32> = Vec::new();
        let num_str = num.to_string();
        
        for digit in num_str.chars() {
            let digit_num = digit.to_digit(10).unwrap() as i32;
            if digit_num % 2 == 0 {
                even_list.push(digit_num);
            } else {
                odd_list.push(digit_num);
            }
        }
        
        even_list.sort_by(|a, b| b.cmp(a));
        odd_list.sort_by(|a, b| b.cmp(a));
        
        let mut result: Vec<i32> = Vec::new();
        let mut even_iter = even_list.iter();
        let mut odd_iter = odd_list.iter();
        
        for digit in num_str.chars() {
            let digit_num = digit.to_digit(10).unwrap() as i32;
            if digit_num % 2 == 0 {
                result.push(*even_iter.next().unwrap());
            } else {
                result.push(*odd_iter.next().unwrap());
            }
        }
        
        let result_str: String = result.iter().map(|x| x.to_string()).collect();
        result_str.parse().unwrap()
    }
}
