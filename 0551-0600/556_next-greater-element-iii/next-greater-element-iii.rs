
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = n.to_string().chars().collect::<Vec<char>>();
        let len = digits.len();
        
        let mut i = len - 2;
        while i >= 0 && digits[i] >= digits[i + 1] {
            i -= 1;
        }
        
        if i < 0 {
            return -1;
        }
        
        let mut j = len - 1;
        while j > i && digits[j] <= digits[i] {
            j -= 1;
        }
        
        digits.swap(i, j);
        digits[i + 1..].sort();
        
        let result = digits.iter().collect::<String>().parse::<i32>().unwrap();
        if result > 2_147_483_647 {
            return -1;
        }
        
        result

    }
}
