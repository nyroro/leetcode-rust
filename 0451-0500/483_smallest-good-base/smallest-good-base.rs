
impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n: u64 = n.parse().unwrap();
        let mut res: u64 = 0;
        for m in (60..1).rev() {
            let mut l: u64 = 2;
            let mut r: u64 = n - 1;
            while l <= r {
                let mid: u64 = l + (r - l) / 2;
                let sum: u64 = (1..m).fold(1, |acc, _| acc * mid + 1);
                if sum == n {
                    res = mid;
                    break;
                } else if sum < n {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
            if res != 0 {
                break;
            }
        }
        res.to_string()
    }
}
