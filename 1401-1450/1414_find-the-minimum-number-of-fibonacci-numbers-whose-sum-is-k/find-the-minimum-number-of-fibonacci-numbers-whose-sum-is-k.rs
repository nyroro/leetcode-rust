
impl Solution {
    pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
        let mut fib = vec![1, 1];
        let mut i = 2;
        while fib[i - 1] < k {
            fib.push(fib[i - 1] + fib[i - 2]);
            i += 1;
        }
        
        let mut count = 0;
        let mut sum = k;
        let mut index = fib.len() - 1;
        
        while sum > 0 {
            if fib[index] <= sum {
                sum -= fib[index];
                count += 1;
            }
            index -= 1;
        }
        
        count

    }
}
