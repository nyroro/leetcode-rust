
impl Solution {
    pub fn maximum_good(statements: Vec<Vec<i32>>) -> i32 {
        let n = statements.len();
        let mut max_good = 0;
        
        // 定义一个辅助函数，用于检查特定的位掩码是否有效

        let check = |mask: usize| -> i32 {
            let mut cnt = 0;
            for i in 0..n {
                if (mask >> i) & 1 == 1 {
                    let mut valid = true;
                    for j in 0..n {
                        if statements[i][j] == 2 {
                            continue;
                        }
                        if statements[i][j] == 0 && (mask >> j) & 1 == 1 {
                            valid = false;
                            break;
                        } else if statements[i][j] == 1 && (mask >> j) & 1 == 0 {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        cnt += 1;
                    }
                }
            }
            cnt

        };
        
        // 遍历所有可能的位掩码

        for mask in 1..(1 << n) {
            max_good = max(max_good, check(mask));
        }
        
        max_good

    }
    
    // 辅助函数，用于比较两个整数的大小

    fn max(a: i32, b: i32) -> i32 {
        if a > b {
            a

        } else {
            b

        }
    }
}
