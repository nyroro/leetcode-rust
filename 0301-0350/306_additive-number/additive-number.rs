
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        fn is_valid_sequence(num: &str, first: &str, second: &str) -> bool {
            if num.is_empty() {
                return true;
            }
            let sum = (first.parse::<u128>().unwrap() + second.parse::<u128>().unwrap()).to_string();
            if !num.starts_with(&sum) {
                return false;
            }
            is_valid_sequence(&num[sum.len()..], second, &sum)
        }
        
        let n = num.len();
        for i in 1..=n / 2 {
            for j in 1..=(n - i) / 2 {
                let first = &num[0..i];
                let second = &num[i..i + j];
                if (first.starts_with("0") && first != "0") || (second.starts_with("0") && second != "0") {
                    continue;
                }
                if is_valid_sequence(&num[i + j..], first, second) {
                    return true;
                }
            }
        }
        false

    }
}
