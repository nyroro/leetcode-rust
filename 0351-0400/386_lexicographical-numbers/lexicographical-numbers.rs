
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 1..10 {
            Solution::dfs(i, n, &mut result);
        }
        result

    }
    
    fn dfs(num: i32, n: i32, result: &mut Vec<i32>) {
        if num > n {
            return;
        }
        result.push(num);
        for i in 0..10 {
            let next_num = num * 10 + i;
            if next_num > n {
                return;
            }
            Solution::dfs(next_num, n, result);
        }
    }
}
