
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut num_str = num.to_string().chars().collect::<Vec<char>>();
        let mut last_occurrence = vec![0; 10];

        for i in 0..num_str.len() {
            let digit = num_str[i].to_digit(10).unwrap() as usize;
            last_occurrence[digit] = i;
        }

        for i in 0..num_str.len() {
            let digit = num_str[i].to_digit(10).unwrap() as usize;
            for j in (digit + 1..=9).rev() {
                if last_occurrence[j] > i {
                    num_str.swap(i, last_occurrence[j]);
                    return num_str.iter().collect::<String>().parse().unwrap();
                }
            }
        }

        num

    }
}
