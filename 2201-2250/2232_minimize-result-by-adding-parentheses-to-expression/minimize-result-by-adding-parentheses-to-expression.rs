
impl Solution {
    pub fn minimize_result(expression: String) -> String {
        let x = expression.find('+').unwrap();
        let n = expression.len();
        let mut m = i32::MAX;
        let mut ret = String::new();
        
        for i in 0..x {
            let a = if i > 0 { expression[..i].parse::<i32>().unwrap() } else { 1 };
            let b = expression[i..x].parse::<i32>().unwrap();
            
            for j in (x+1)..n {
                let c = expression[(x+1)..j+1].parse::<i32>().unwrap();
                let d = if j+1 < n { expression[(j+1)..].parse::<i32>().unwrap() } else { 1 };
                
                if a*(b+c)*d < m {
                    m = a*(b+c)*d;
                    ret = format!("{}({}){}", &expression[..i], &expression[i..j+1], &expression[j+1..]);
                }
            }
        }
        
        ret

    }
}
