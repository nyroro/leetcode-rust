
impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut nums: Vec<char> = (b'1'..=b'0' + n as u8).map(|c| c as char).collect();
        let mut result = String::new();
        let mut k = k - 1;

        let mut factorial = vec![1; n as usize];
        for i in 1..n as usize {
            factorial[i] = factorial[i - 1] * i as i32;
        }

        for i in (0..n as usize).rev() {
            let index = (k / factorial[i]) as usize;
            result.push(nums.remove(index));
            k %= factorial[i];
        }

        result

    }
}
