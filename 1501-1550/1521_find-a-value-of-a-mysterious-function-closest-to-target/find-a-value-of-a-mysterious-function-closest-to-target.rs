
impl Solution {
    pub fn closest_to_target(arr: Vec<i32>, target: i32) -> i32 {
        let mut result = std::i32::MAX;
        let mut s = std::collections::HashSet::new();
        for &x in &arr {
            let mut new_s = std::collections::HashSet::new();
            new_s.insert(x);
            for &y in &s {
                new_s.insert(y & x);
            }
            s = new_s;
            for &y in &s {
                result = result.min((y - target).abs());
            }
        }
        result

    }
}
