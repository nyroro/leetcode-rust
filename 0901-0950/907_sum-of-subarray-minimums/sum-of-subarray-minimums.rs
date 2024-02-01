
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut stack: Vec<usize> = Vec::new();
        let modulo = 1000000007;

        for (i, &num) in arr.iter().enumerate() {
            while !stack.is_empty() && num < arr[*stack.last().unwrap()] {
                let top = stack.pop().unwrap();
                let left = if stack.is_empty() { 0 } else { stack.last().unwrap() + 1 };
                let right = i;
                let contribution = ((arr[top] as i64) * ((top - left + 1) as i64) * ((right - top) as i64)) % (modulo as i64);
                result = (result + contribution as i32) % modulo;
            }
            stack.push(i);
        }

        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let left = if stack.is_empty() { 0 } else { stack.last().unwrap() + 1 };
            let right = arr.len();
            let contribution = ((arr[top] as i64) * ((top - left + 1) as i64) * ((right - top) as i64)) % (modulo as i64);
            result = (result + contribution as i32) % modulo;
        }

        result

    }
}
