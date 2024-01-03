
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        if n == 0 {
            return vec![0];
        }
        
        let prev = Solution::gray_code(n - 1);
        let rev: Vec<i32> = prev.iter().map(|&x| x + (1 << (n - 1))).rev().collect();
        
        let mut result = prev.clone();
        result.extend(rev);
        
        result

    }
}
