
impl Solution {
    pub fn count_days_together(arrive_alice: String, leave_alice: String, arrive_bob: String, leave_bob: String) -> i32 {
        // Convert date to nth day of the year

        fn get_day_of_year(date: &str) -> i32 {
            let days_in_month = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let parts: Vec<&str> = date.split("-").collect();
            let month = parts[0].parse::<usize>().unwrap();
            let day = parts[1].parse::<usize>().unwrap();
            let mut day_of_year = day as i32;
            for i in 0..(month - 1) {
                day_of_year += days_in_month[i] as i32;
            }
            day_of_year

        }

        // Convert Alice and Bob's dates to day of the year

        let a_start = get_day_of_year(&arrive_alice);
        let a_end = get_day_of_year(&leave_alice);
        let b_start = get_day_of_year(&arrive_bob);
        let b_end = get_day_of_year(&leave_bob);

        // Find the overlapping days

        let start = a_start.max(b_start);
        let end = a_end.min(b_end);

        // Calculate the total days together

        let days_together = (end - start + 1).max(0);
        days_together

    }
}
