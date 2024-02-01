
impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let sides: Vec<&str> = equation.split('=').collect();
        let left_side = sides[0];
        let right_side = sides[1];

        let (ax, an) = Solution::calculate_coefficients(left_side);
        let (bx, bn) = Solution::calculate_coefficients(right_side);

        if ax == bx {
            if an == bn {
                return "Infinite solutions".to_string();
            } else {
                return "No solution".to_string();
            }
        } else {
            let x_val = (bn - an) / (ax - bx);
            return format!("x={}", x_val);
        }
    }

    fn calculate_coefficients(s: &str) -> (i32, i32) {
        let mut ax = 0;
        let mut an = 0;
        let mut num = String::new();
        let mut sign = 1;
        for ch in s.chars() {
            match ch {
                'x' => {
                    if num.is_empty() {
                        ax += sign;
                    } else {
                        ax += sign * num.parse::<i32>().unwrap();
                    }
                    num.clear();
                    sign = 1;
                }
                '+' | '-' => {
                    if !num.is_empty() {
                        an += sign * num.parse::<i32>().unwrap();
                        num.clear();
                    }
                    if ch == '-' {
                        sign = -1;
                    } else {
                        sign = 1;
                    }
                }
                _ => {
                    num.push(ch);
                }
            }
        }
        if !num.is_empty() {
            an += sign * num.parse::<i32>().unwrap();
        }
        (ax, an)  // Return the tuple (ax, an)
    }
}
