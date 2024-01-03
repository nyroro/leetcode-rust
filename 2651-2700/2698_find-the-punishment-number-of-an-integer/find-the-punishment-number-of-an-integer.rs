
impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let range = 1..=n;
        range.into_iter()
            .filter(Self::valid)
            .map(|a| a * a)
            .sum()
    }

    fn valid(s: &i32) -> bool {
        let mut temp = s * s;
        let mut digits = vec![];
        while temp > 0 {
            digits.push(temp % 10);
            temp = temp / 10;
        }
        digits.reverse();
        return Self::helper(&digits, 0, 0, s);
    }

    fn helper(d: &Vec<i32>, start: usize, sum: i32, target: &i32) -> bool {
        if start == d.len() {
            return sum == *target;
        }
        let mut partition = 0;
        let mut i = start;
        loop {
            if i == d.len() {
                return false;
            }
            partition = partition * 10 + d[i];
            if Self::helper(d, i + 1, sum + partition, target) {
                return true;
            }
            i += 1;
        }
    }
}
