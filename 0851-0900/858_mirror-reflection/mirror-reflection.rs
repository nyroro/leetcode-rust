
impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let g = gcd(p, q);
        let p = p / g;
        let q = q / g;
        if p % 2 == 0 && q % 2 == 1 {
            return 2;
        } else if p % 2 == 1 && q % 2 == 1 {
            return 1;
        } else {
            return 0;
        }
    }
    
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a

    }
}
