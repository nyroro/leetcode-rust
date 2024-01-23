
use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b > 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a

        }

        let mut modified_nums: Vec<i32> = nums.iter().map(|&x| gcd(x, k)).collect();
        let mut table: HashMap<i32, i64> = HashMap::new();
        let mut result: i64 = 0;

        for &t in modified_nums.iter() {
            let z = k / t;
            result += *table.get(&z).unwrap_or(&0);
            for j in 1..=(t / 2).max(2) {
                if j * j > t {
                    break;
                }
                if t % j != 0 {
                    continue;
                }
                let x = j;
                let y = t / x;
                *table.entry(x).or_insert(0) += 1;
                if y != x {
                    *table.entry(y).or_insert(0) += 1;
                }
            }
        }

        result

    }
}
