
impl Solution {
    pub fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
        let n = statements.len();
        let mut max_good = 0;
        
        for mask in 0..(1 << n) {
            let mut is_good = vec![false; n];
            let mut good_count = 0;
            let mut valid = true;
            
            for i in 0..n {
                if (mask >> i) & 1 == 1 {
                    is_good[i] = true;
                    good_count += 1;
                }
            }
            
            for i in 0..n {
                if !is_good[i] {
                    continue;
                }
                for j in 0..n {
                    if statements[i][j] == 0 && is_good[j] {
                        valid = false;
                        break;
                    }
                    if statements[i][j] == 1 && !is_good[j] {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
            
            if valid {
                max_good = max_good.max(good_count);
            }
        }
        
        max_good

    }
}
