


impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut length = 0;
        let mut t = target;
        while t > 0 {
            t /= 2;
            length += 1;
        }

        if max_doubles >= length {
            let mut ret = 0;
            let mut t = target;
            while t > 0 {
                ret += 1;
                t -= t & -t;
            }
            return ret + length - 2;
        } else {
            let mut ret = max_doubles;
            let l = target >> max_doubles;
            let mut r = target & ((1 << max_doubles) - 1);
            while r > 0 {
                ret += 1;
                r -= r & -r;
            }
            ret += l - 1;
            return ret;
        }
    }
}
