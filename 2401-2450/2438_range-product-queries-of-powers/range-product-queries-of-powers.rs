


const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut powers = vec![];
        let mut num = n;
        while num > 0 {
            let power = (num & -num) as i32;
            powers.push(power);
            num -= power;
        }

        let mut result = vec![];
        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            let mut product = 1;
            for i in left..=right {
                product = (product as i64 * powers[i] as i64 % MOD) as i32;
            }
            result.push(product);
        }

        result

    }
}
