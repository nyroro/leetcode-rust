
impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let nn: i64 = n.parse().unwrap();
        let mut ret: Vec<i64> = vec![];
        
        if n.len() == 1 {
            return (nn - 1).to_string();
        }
        
        let a: i64 = "9".repeat(n.len() - 1).parse().unwrap();
        ret.push(a);
        
        let b: i64 = format!("1{}1", "0".repeat(n.len() - 1)).parse().unwrap();
        if (ret[0] - nn).abs() > (b - nn).abs() {
            ret[0] = b;
        }
        
        let mut arr: Vec<char> = n.chars().collect();
        
        fn dfs(i: usize, arr: &mut Vec<char>, nn: i64, ret: &mut Vec<i64>) {
            if i >= arr.len() / 2 + arr.len() % 2 {
                let val: i64 = arr.iter().collect::<String>().parse().unwrap();
                if val == nn {
                    return;
                }
                if (ret[0] - nn).abs() > (val - nn).abs() {
                    ret[0] = val;
                } else if (ret[0] - nn).abs() == (val - nn).abs() && val < ret[0] {
                    ret[0] = val;
                }
                return;
            }
            let now: u8 = arr[i] as u8 - b'0';
            if now > 0 {
                arr[i] = ((now - 1) + b'0') as char;
                arr[arr.len() - i - 1] = ((now - 1) + b'0') as char;
                arr[(i + 1)..(arr.len() - i - 1)].iter_mut().for_each(|x| *x = '9');
                dfs(arr.len() + 1, arr, nn, ret);
            }
            if now < 9 {
                arr[i] = ((now + 1) + b'0') as char;
                arr[arr.len() - i - 1] = ((now + 1) + b'0') as char;
                arr[(i + 1)..(arr.len() - i - 1)].iter_mut().for_each(|x| *x = '0');
                dfs(arr.len() + 1, arr, nn, ret);
            }
            arr[i] = (now + b'0') as char;
            arr[arr.len() - i - 1] = (now + b'0') as char;
            dfs(i + 1, arr, nn, ret);
        }
        
        dfs(0, &mut arr, nn, &mut ret);
        
        ret[0].to_string()
    }
}
