
impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let sides: Vec<&str> = equation.split('=').collect();
        let pattern = regex::Regex::new(r"([+-]*)([\dx]+)").unwrap();

        fn calculate_coefficients(s: &str, pattern: &regex::Regex) -> (i32, i32) {
            let mut ax = 0;
            let mut an = 0;
            for cap in pattern.captures_iter(s) {
                let sign = cap.get(1).unwrap().as_str();
                let val = cap.get(2).unwrap().as_str();
                if val.ends_with('x') {
                    let cnt = if val.len() > 1 { val[..val.len() - 1].parse().unwrap() } else { 1 };
                    if sign == "-" {
                        ax -= cnt;
                    } else {
                        ax += cnt;
                    }
                } else {
                    let num = val.parse().unwrap();
                    if sign == "-" {
                        an -= num;
                    } else {
                        an += num;
                    }
                }
            }
            (ax, an)
        }

        let (ax, an) = calculate_coefficients(sides[0], &pattern);
        let (bx, bn) = calculate_coefficients(sides[1], &pattern);

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
}
