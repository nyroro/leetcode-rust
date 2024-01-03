
impl Solution {
    pub fn num_friend_requests(ages: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut age_count = vec![0; 121];

        for &age in &ages {
            age_count[age as usize] += 1;
        }

        for i in 1..=120 {
            for j in (i / 2 + 8)..=i {
                count += age_count[i as usize] * (age_count[j as usize] - (i == j) as i32);
            }
        }

        count

    }
}
