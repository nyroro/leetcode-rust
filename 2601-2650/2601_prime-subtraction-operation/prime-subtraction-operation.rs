
impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let max_num = *nums.iter().max().unwrap() as usize + 1;
        let mut primes = vec![true; max_num];
        let mut prime_list = Vec::new();

        for i in 2..max_num {
            if primes[i] {
                prime_list.push(i as i32);
                let mut j = i * i;
                while j < max_num {
                    primes[j] = false;
                    j += i;
                }
            }
        }

        let mut prev = 0;
        for &num in nums.iter() {
            if num <= prev {
                return false;
            }
            let mut next = num;
            for &p in prime_list.iter() {
                if num - p > prev {
                    next = num - p;
                } else {
                    break;
                }
            }
            prev = next;
        }

        true

    }
}
