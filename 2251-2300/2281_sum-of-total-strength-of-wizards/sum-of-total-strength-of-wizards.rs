


impl Solution {
    pub fn total_strength(strength: Vec<i32>) -> i32 {
        let modulo: i64 = 1_000_000_007;
        let n = strength.len();
        
        let mut prefix = vec![0; n + 1];
        for i in 0..n {
            prefix[i + 1] = (prefix[i] + strength[i] as i64) % modulo;
        }
        
        let mut prefix_sum = vec![0; n + 2];
        for i in 0..=n {
            prefix_sum[i + 1] = (prefix_sum[i] + prefix[i]) % modulo;
        }
        
        let mut left = vec![-1; n];
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..n {
            while !stack.is_empty() && strength[*stack.last().unwrap()] >= strength[i] {
                stack.pop();
            }
            left[i] = if stack.is_empty() { -1 } else { stack.last().unwrap().clone() as i32 };
            stack.push(i);
        }
        
        let mut right = vec![n as i32; n];
        stack.clear();
        for i in (0..n).rev() {
            while !stack.is_empty() && strength[*stack.last().unwrap()] > strength[i] {
                stack.pop();
            }
            right[i] = if stack.is_empty() { n as i32 } else { stack.last().unwrap().clone() as i32 };
            stack.push(i);
        }
        
        let mut res: i64 = 0;
        for i in 0..n {
            res += ((prefix_sum[right[i] as usize + 1] - prefix_sum[i as usize + 1]) * (i as i64 - left[i] as i64) % modulo + modulo * 2 - 
                   (prefix_sum[i as usize + 1] - prefix_sum[(left[i] + 1) as usize]) * (right[i] as i64 - i as i64) % modulo) % modulo * strength[i] as i64 % modulo;
            res %= modulo;
        }
        
        res as i32

    }
}
