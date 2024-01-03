
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize];
        for num in arr {
            let rem = (num % k + k) % k;
            count[rem as usize] += 1;
        }
        if count[0] % 2 != 0 {
            return false;
        }
        for i in 1..(k / 2 + 1) {
            if count[i as usize] != count[(k - i) as usize] {
                return false;
            }
        }
        return true;
    }
}
