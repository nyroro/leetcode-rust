
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        // Convert the integer to a string

        let s1 = num.to_string();
        let mut sb1 = String::new();
        let mut sb2 = String::new();
        let mut i = 0;

        // Find the first non-'9' digit

        while i < s1.len() {
            let c = s1.chars().nth(i).unwrap();
            let delta = c as i32 - '9' as i32;
            if delta != 0 {
                break;
            }
            i += 1;
        }

        // Construct the maximum number a

        for j in 0..i {
            sb1.push('9');
        }
        for j in i..s1.len() {
            if s1.chars().nth(j).unwrap() == s1.chars().nth(i).unwrap() {
                sb1.push('9');
            } else {
                sb1.push(s1.chars().nth(j).unwrap());
            }
        }

        i = 0;
        // Find the first non-'1' or '0' digit

        while i < s1.len() {
            let c = s1.chars().nth(i).unwrap();
            let delta = c as i32 - '1' as i32;
            if delta >= 1 {
                break;
            }
            i += 1;
        }

        // Determine the replacement character

        let r = if i == 0 || i == s1.len() {
            '1'
        } else {
            '0'
        };

        // Construct the minimum number b

        for j in 0..i {
            sb2.push(s1.chars().nth(j).unwrap());
        }
        for j in i..s1.len() {
            if s1.chars().nth(j).unwrap() == s1.chars().nth(i).unwrap() {
                sb2.push(r);
            } else {
                sb2.push(s1.chars().nth(j).unwrap());
            }
        }

        // Return the difference between a and b

        sb1.parse::<i32>().unwrap() - sb2.parse::<i32>().unwrap()
    }
}
