
impl Solution {
    pub fn closest_divisors(num: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut diff = i32::MAX;

        for i in 1..=((num + 2) as f64).sqrt() as i32 {
            let a = (num + 1) / i;
            let b = (num + 2) / i;

            if a * i == num + 1 {
                if (a - i).abs() < diff {
                    diff = (a - i).abs();
                    res = vec![i, a];
                }
            }

            if b * i == num + 2 {
                if (b - i).abs() < diff {
                    diff = (b - i).abs();
                    res = vec![i, b];
                }
            }
        }

        res

    }
}
