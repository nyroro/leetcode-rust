
impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let n = arr.len() as i32;
        for i in 0..n - m * k + 1 {
            let mut valid = true;
            for j in 0..m {
                for l in 1..k {
                    if arr[(i + j) as usize] != arr[(i + j + l * m) as usize] {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
            if valid {
                return true;
            }
        }
        false

    }
}
