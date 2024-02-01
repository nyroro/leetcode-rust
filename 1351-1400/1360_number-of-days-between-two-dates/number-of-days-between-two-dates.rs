


impl Solution {
    fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    fn get_days(date: &str) -> i32 {
        let days_in_month = [
            0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31

        ];

        let date_parts: Vec<i32> = date.split("-")
            .map(|s| s.parse().unwrap())
            .collect();

        let year = date_parts[0];
        let month = date_parts[1];
        let day = date_parts[2];

        let mut total = 0;

        for y in 1971..year {
            if Solution::is_leap_year(y) {
                total += 366;
            } else {
                total += 365;
            }
        }

        for m in 1..month {
            total += days_in_month[m as usize];
        }

        if month > 2 && Solution::is_leap_year(year) {
            total += 1;
        }

        total += day;

        total

    }

    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        let total_days1 = Solution::get_days(&date1);
        let total_days2 = Solution::get_days(&date2);

        (total_days1 - total_days2).abs()
    }
}
