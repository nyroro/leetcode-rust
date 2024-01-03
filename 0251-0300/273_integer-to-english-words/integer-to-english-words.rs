
impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        } else {
            return Self::convert_to_words(num);
        }
    }

    fn convert_to_words(num: i32) -> String {
        let mut result = String::new();
        let units = vec!["", "Thousand", "Million", "Billion"];
        let mut num = num;
        let mut i = 0;

        while num > 0 {
            if num % 1000 != 0 {
                result = Self::helper(num % 1000) + " " + units[i] + " " + &result;
            }
            num /= 1000;
            i += 1;
        }

        result.trim().to_string()
    }

    fn helper(num: i32) -> String {
        let below_20 = vec![
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
            "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen",
            "Eighteen", "Nineteen",
        ];
        let tens = vec!["", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];

        if num == 0 {
            return "".to_string();
        } else if num < 20 {
            return below_20[num as usize].to_string();
        } else if num < 100 {
            return (tens[num as usize / 10].to_string() + " " + &Self::helper(num % 10)).trim().to_string();
        } else {
            return (below_20[num as usize / 100].to_string() + " Hundred " + &Self::helper(num % 100)).trim().to_string();
        }
    }
}
