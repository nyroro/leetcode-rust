
impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        fn gcd(mut a: i32, mut b: i32) -> i32 {
            if a == b {
                return a;
            }
            if a < b {
                std::mem::swap(&mut a, &mut b);
            }
            while b > 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a

        }

        for &t in nums.iter() {
            if ret.is_empty() {
                ret.push(t);
            } else {
                let mut g = gcd(*ret.last().unwrap(), t);
                if g == 1 {
                    ret.push(t);
                } else {
                    let mut v = ret.pop().unwrap();
                    let mut new_val = v / g * t;
                    while !ret.is_empty() && gcd(*ret.last().unwrap(), new_val) > 1 {
                        v = ret.pop().unwrap();
                        new_val = v / gcd(v, new_val) * new_val;
                    }
                    ret.push(new_val);
                }
            }
        }
        ret

    }
}
