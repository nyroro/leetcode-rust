
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut stack: Vec<usize> = Vec::new();
        let modulo = 1000000007;

        for (i, &num) in arr.iter().enumerate() {
            while !stack.is_empty() && num <= arr[*stack.last().unwrap()] {
                let top = stack.pop().unwrap();
                let contribution = (arr[top] * (i - top) * (top - if stack.is_empty() { 0 } else { stack.last().unwrap() + 1 })) % modulo;
                result = (result + contribution) % modulo;
            }
            stack.push(i);
        }

        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let contribution = (arr[top] * (arr.len() - top) * (top - if stack.is_empty() { 0 } else { stack.last().unwrap() + 1 })) % modulo;
            result = (result + contribution) % modulo;
        }

        result as i32

    }
}
