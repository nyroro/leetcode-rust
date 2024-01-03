
impl Solution {
    pub fn max_value(n: String, x: i32) -> String {
        let mut n = n.chars().collect::<Vec<char>>();
        let mut negative = false;
        if n[0] == '-' {
            negative = true;
            n.remove(0);
        }
        if !negative {
            for i in 0..n.len() {
                if x.to_string().parse::<char>().unwrap() > n[i] {
                    n.insert(i, x.to_string().parse::<char>().unwrap());
                    break;
                }
                if i == n.len() - 1 {
                    n.push(x.to_string().parse::<char>().unwrap());
                }
            }
        } else {
            for i in 0..n.len() {
                if x.to_string().parse::<char>().unwrap() < n[i] {
                    n.insert(i, x.to_string().parse::<char>().unwrap());
                    break;
                }
                if i == n.len() - 1 {
                    n.push(x.to_string().parse::<char>().unwrap());
                }
            }
        }
        if negative {
            n.insert(0, '-');
        }
        n.iter().collect()
    }
}
