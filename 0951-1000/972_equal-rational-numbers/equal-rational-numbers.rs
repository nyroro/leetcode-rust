
// Define a function to convert the input string

fn convert(input: &str) -> String {
    if input.contains('(') {
        let rep_part = &input[input.find('(').unwrap() + 1..input.find(')').unwrap()];
        let mut new_input = input.replace("(", "").replace(")", "");
        new_input.push_str(&rep_part.repeat(8)); // Repeat the repeating part 8 times

        new_input

    } else {
        input.to_string()
    }
}

// Define a function to round down the number

fn round_down(num: f64, decimals: u32) -> f64 {
    (num * 10_f64.powi(decimals as i32) + 0.5).floor() / 10_f64.powi(decimals as i32)
}

// Implement the solution function

impl Solution {
    pub fn is_rational_equal(s: String, t: String) -> bool {
        let s_converted = convert(&s);
        let t_converted = convert(&t);
        let s_rounded = round_down(s_converted.parse::<f64>().unwrap(), 8);
        let t_rounded = round_down(t_converted.parse::<f64>().unwrap(), 8);
        
        s_rounded == t_rounded

    }
}
