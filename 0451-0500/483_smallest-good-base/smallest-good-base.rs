
impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n: u64 = n.parse().unwrap();
        let max_m = (n as f64).log2() as u32 + 1;
        for m in (2..=max_m).rev() {
            let k = (n as f64).powf(1.0 / (m - 1) as f64) as u64;
            if (1..m).fold((1, 1), |(sum, pow), _| (sum + pow * k, pow * k)).0 == n {
                return k.to_string();
            }
        }
        (n - 1).to_string()
    }
}
