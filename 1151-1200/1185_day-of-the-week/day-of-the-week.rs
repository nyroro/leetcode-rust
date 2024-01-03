
impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let days_of_week = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let year = if month < 3 { year - 1 } else { year };
        let day_index = (year + year / 4 - year / 100 + year / 400 + t[(month - 1) as usize] + day) % 7;
        days_of_week[day_index as usize].to_string()
    }
}
