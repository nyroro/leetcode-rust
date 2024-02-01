
impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let n = chalk.len();
        let mut total = k;
        
        for i in 0..n {
            total -= chalk[i];
            if total < 0 {
                return i as i32;
            }
        }
        
        Solution::chalk_replacer(chalk, total % chalk.iter().sum())
    }
}
