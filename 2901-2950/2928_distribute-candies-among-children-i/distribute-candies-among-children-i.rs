
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i32 {
        let mut count = 0;
        let mut result = 0;
        let mut candies = vec![0; 3];
        Self::backtrack(0, n, limit, &mut count, &mut result, &mut candies);
        result

    }
    
    fn backtrack(start: usize, n: i32, limit: i32, count: &mut i32, result: &mut i32, candies: &mut Vec<i32>) {
        if start == 3 {
            if *count == n {
                *result += 1;
            }
            return;
        }
        
        for i in 0..=limit {
            if *count + i <= n {
                candies[start] = i;
                *count += i;
                Self::backtrack(start + 1, n, limit, count, result, candies);
                *count -= i;
            }
        }
    }
}
