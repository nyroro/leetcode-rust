


impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        let mut p = std::i32::MAX;
        let mut res = 0;

        for i in 0..colors.len() {
            if colors[i] != colors[0] {
                res = i as i32;
                p = p.min(i as i32);
            } else {
                res = res.max(i as i32 - p);
            }
        }

        res

    }
}
