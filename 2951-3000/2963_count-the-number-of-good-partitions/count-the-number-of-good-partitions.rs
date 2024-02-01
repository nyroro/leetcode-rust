
use std::collections::HashSet;

impl Solution {
    pub fn number_of_good_partitions(nums: Vec<i32>) -> i32 {
        const N: i32 = 1_000_000_007;

        // Function to perform binary exponentiation

        fn binary(a: i32, b: i32) -> i32 {
            let mut res = 1;
            let mut a = a;
            let mut b = b;
            while b > 0 {
                if b & 1 == 1 {
                    res = ((res % N) * (a % N)) % N;
                }
                a = ((a % N) * (a % N)) % N;
                res = res % N;
                b >>= 1;
            }
            res

        }

        let mut s: HashSet<(i32, i32)> = HashSet::new(); // HashSet to store frequency and element

        let mut m: std::collections::HashMap<i32, i32> = std::collections::HashMap::new(); // HashMap to store frequency of elements


        // Count frequency of each element in the nums array

        for &x in &nums {
            *m.entry(x).or_insert(0) += 1;
        }

        let mut count = 0;
        let mut i = 0;
        let n = nums.len();

        // Iterate through the array

        while i < n {
            s.remove(&(-1 * m[&nums[i]], nums[i])); // Remove frequency and element pair from set

            m.entry(nums[i]).and_modify(|e| *e -= 1); // Decrease frequency of the current element


            // If the element still has frequency left, insert it back into the set

            if m[&nums[i]] > 0 {
                s.insert((-1 * m[&nums[i]], nums[i]));
            }

            i += 1;

            // If the set is empty, increment the count

            if s.is_empty() {
                count += 1;
            }
        }

        // Calculate the result using binary exponentiation

        binary(2, count - 1)
    }
}
