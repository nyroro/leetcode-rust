


impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut curr: Vec<String> = Vec::new();
        let mut stack: Vec<(Vec<String>, Vec<String>)> = Vec::new();

        for x in expression.chars() {
            if x.is_ascii_lowercase() {
                if !curr.is_empty() {
                    for i in curr.iter_mut() {
                        i.push(x);
                    }
                } else {
                    curr.push(x.to_string());
                }
            } else if x == '{' {
                stack.push((res.clone(), curr.clone()));
                res.clear();
                curr.clear();
            } else if x == '}' {
                if let Some((pre_res, pre_curr)) = stack.pop() {
                    for i in curr.iter() {
                        res.push(i.clone());
                    }
                    curr = multiply(&pre_curr, &res);
                    res = pre_res;
                }
            } else if x == ',' {
                for i in curr.iter() {
                    res.push(i.clone());
                }
                curr.clear();
            }
        }

        for i in curr.iter() {
            res.push(i.clone());
        }

        res.sort();
        res.dedup();

        res

    }
}

fn multiply(a: &Vec<String>, b: &Vec<String>) -> Vec<String> {
    if a.is_empty() {
        return b.clone();
    }
    if b.is_empty() {
        return a.clone();
    }
    let mut ans: Vec<String> = Vec::new();
    for i in a.iter() {
        for j in b.iter() {
            ans.push(format!("{}{}", i, j));
        }
    }
    ans

}
