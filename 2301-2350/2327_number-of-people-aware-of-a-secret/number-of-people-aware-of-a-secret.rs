


impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        let modulo = 1_000_000_007;
        let mut people = vec![0; n as usize + 1];
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

        people[n as usize]
    }
}

fn main() {
    let n1 = 6;
    let delay1 = 2;
    let forget1 = 4;
    let result1 = Solution::people_aware_of_secret(n1, delay1, forget1);
    println!("Result 1: {}", result1);  // Output: 5


    let n2 = 4;
    let delay2 = 1;
    let forget2 = 3;
    let result2 = Solution::people_aware_of_secret(n2, delay2, forget2);
    println!("Result 2: {}", result2);  // Output: 6

}
