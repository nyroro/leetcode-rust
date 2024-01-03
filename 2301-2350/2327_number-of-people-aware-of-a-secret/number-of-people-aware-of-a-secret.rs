


impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let modulo = 1_000_000_007;
        let mut people = vec![0; (n + 1) as usize];
        people[1] = 1;

        for day in 2..=n {
            people[day as usize] = people[(day - 1) as usize];
            if day >= delay {
                people[day as usize] = (people[day as usize] + people[(day - delay) as usize]) % modulo;
            }
            if day >= forget {
                people[day as usize] = (people[day as usize] - people[(day - forget) as usize] + modulo) % modulo;
            }
        }

        (people[n as usize] - people[(n - forget) as usize] + modulo) % modulo

    }
}
