
impl Solution {
    pub fn reformat_date(date: String) -> String {
        // Split the date string by space

        let parts: Vec<&str> = date.split_whitespace().collect();
        
        // Extract day, month, and year

        let day = parts[0].trim_matches(char::is_alphabetic);
        let month = match parts[1] {
            "Jan" => "01",
            "Feb" => "02",
            "Mar" => "03",
            "Apr" => "04",
            "May" => "05",
            "Jun" => "06",
            "Jul" => "07",
            "Aug" => "08",
            "Sep" => "09",
            "Oct" => "10",
            "Nov" => "11",
            "Dec" => "12",
            _ => panic!("Invalid month"),
        };
        let year = parts[2];
        
        // Format the date in YYYY-MM-DD

        format!("{}-{}-{}", year, month, day)
    }
}
