
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut result = Vec::new();
        let mut i = 0;
        while x.pow(i) <= bound {
            let mut j = 0;
            while x.pow(i) + y.pow(j) <= bound {
                result.push(x.pow(i) + y.pow(j));
                if y == 1 {
                    break;
                }
                j += 1;
            }
            if x == 1 {
                break;
            }
            i += 1;
        }
        result.sort();
        result.dedup();
        result

    }
}
