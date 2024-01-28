
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // Function to perform binary exponentiation

        fn binary(mut a: i64, mut b: i64) -> i64 {
            let mut res = 1;
            a %= MOD;
            while b > 0 {
                if b % 2 == 1 {
                    res = (res * a) % MOD;
                }
                a = (a * a) % MOD;
                b /= 2;
            }
            res

        }

        let mut s: HashSet<(i32, i32)> = HashSet::new(); // HashSet to store frequency and element

        let mut m: HashMap<i32, i32> = HashMap::new(); // HashMap to store frequency of elements

        let mut count = 0;
        let n = nums.len();

        // Count frequency of each element in the nums array

        for &x in &nums {
            *m.entry(x).or_insert(0) += 1;
        }

        // Iterate through the array

        for i in 0..n {
            s.remove(&(-1 * m[&nums[i]], nums[i])); // Remove frequency and element pair from set

            m.entry(nums[i]).and_modify(|e| *e -= 1); // Decrease frequency of the current element


            // If the element still has frequency left, insert it back into the set

            if m[&nums[i]] > 0 {
                s.insert((-1 * m[&nums[i]], nums[i]));
            }

            // If the set is empty, increment the count

            if s.is_empty() {
                count += 1;
            }
        }

        // Calculate the result using binary exponentiation

        binary(2, count as i64 - 1) as i32

    }
}
