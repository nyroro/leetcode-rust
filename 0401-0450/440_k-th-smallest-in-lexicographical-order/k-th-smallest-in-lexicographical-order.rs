
impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut curr = 1;
        let mut k = k - 1;

        while k > 0 {
            let steps = Self::cal_steps(n, curr, curr + 1);
            if steps <= k {
                curr += 1;
                k -= steps;
            } else {
                curr *= 10;
                k -= 1;
            }
        }

        curr as i32

    }

    fn cal_steps(n: i32, n1: i32, n2: i32) -> i32 {
        let mut steps = 0;
        let mut n1 = n1;
        let mut n2 = n2;

        while n1 <= n {
            steps += i32::min(n + 1, n2) - n1;
            n1 *= 10;
            n2 *= 10;
        }

        steps

    }
}
