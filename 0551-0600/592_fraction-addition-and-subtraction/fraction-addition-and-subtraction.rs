


impl Solution {
    pub fn gcd(a: i32, b: i32) -> i32 {
        if a % b == 0 {
            b

        } else {
            Solution::gcd(b, a % b)
        }
    }

    pub fn get_num(i: &mut usize, exp: &str) -> i32 {
        let mut temp = String::new();
        while exp.chars().nth(*i).unwrap() != '/' {
            temp.push(exp.chars().nth(*i).unwrap());
            *i += 1;
        }
        temp.parse().unwrap()
    }

    pub fn get_den(i: &mut usize, exp: &str) -> i32 {
        let mut temp = String::new();
        while *i < exp.len() && (exp.chars().nth(*i).unwrap() != '+' && exp.chars().nth(*i).unwrap() != '-') {
            temp.push(exp.chars().nth(*i).unwrap());
            *i += 1;
        }
        let num: i32 = temp.parse().unwrap();
        num

    }

    pub fn fraction_addition(expression: String) -> String {
        let mut s1 = 1;
        let mut i = 0;
        if expression.chars().nth(0).unwrap() == '-' {
            s1 = -1;
            i += 1;
        }
        let mut num1 = Solution::get_num(&mut i, &expression);
        i += 1;
        let mut den1 = Solution::get_den(&mut i, &expression);
        while i < expression.len() {
            let s2 = if expression.chars().nth(i).unwrap() == '-' { -1 } else { 1 };
            i += 1;
            let num2 = Solution::get_num(&mut i, &expression);
            i += 1;
            let den2 = Solution::get_den(&mut i, &expression);
            let lcm = (den2 * den1) / Solution::gcd(den1, den2);
            num1 = (s1 * num1 * (lcm / den1) + s2 * num2 * (lcm / den2));
            den1 = lcm;
            if num1 < 0 {
                s1 = -1;
                num1 *= -1;
            } else {
                s1 = 1;
            }
        }
        let gc = Solution::gcd(num1, den1);
        if gc > 1 {
            num1 = num1 / gc;
            den1 = den1 / gc;
        }
        let mut ans = String::new();
        if s1 == -1 {
            ans.push('-');
        }
        ans.push_str(&num1.to_string());
        ans.push('/');
        ans.push_str(&den1.to_string());
        ans

    }
}
