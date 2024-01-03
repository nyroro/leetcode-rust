
impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let mut result = 0;
        let mut arr = Vec::new();
        let mut num = 1;

        while arr.len() < n as usize {
            let mut valid = true;
            for &x in &arr {
                if x + num == k {
                    valid = false;
                    break;
                }
            }
            if valid {
                arr.push(num);
                result += num;
            }
            num += 1;
        }

        result

    }
}
