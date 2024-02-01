
impl Solution {
    fn count_of_atoms(formula: String) -> String {
        use std::collections::HashMap;
        
        let mut counts: HashMap<String, i32> = HashMap::new();
        let mut stack: Vec<HashMap<String, i32>> = Vec::new();
        let mut i = 0;
        let n = formula.len();
        
        while i < n {
            let c = formula.chars().nth(i).unwrap();
            i += 1;
            if c == '(' {
                stack.push(counts);
                counts = HashMap::new();
            } else if c == ')' {
                let mut temp = 0;
                while i < n && formula.chars().nth(i).unwrap().is_numeric() {
                    temp = temp * 10 + formula.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
                    i += 1;
                }
                if temp == 0 {
                    temp = 1;
                }
                if !stack.is_empty() {
                    let mut prev_counts = stack.pop().unwrap();
                    for (key, val) in counts.iter() {
                        *prev_counts.entry(key.to_string()).or_insert(0) += val * temp;
                    }
                    counts = prev_counts;
                }
            } else {
                let start = i - 1;
                while i < n && formula.chars().nth(i).unwrap().is_ascii_lowercase() {
                    i += 1;
                }
                let name = &formula[start..i];
                let mut temp = 0;
                while i < n && formula.chars().nth(i).unwrap().is_numeric() {
                    temp = temp * 10 + formula.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
                    i += 1;
                }
                if temp == 0 {
                    temp = 1;
                }
                *counts.entry(name.to_string()).or_insert(0) += temp;
            }
        }
        
        let mut elements: Vec<_> = counts.keys().collect();
        elements.sort();
        let mut result = String::new();
        for elem in elements {
            result.push_str(elem);
            if *counts.get(elem).unwrap() > 1 {
                result.push_str(&counts.get(elem).unwrap().to_string());
            }
        }
        result

    }
}
