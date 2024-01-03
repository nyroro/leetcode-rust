
impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut count = 0;
        
        for detail in details {
            let age_str = &detail[11..13];
            let age = age_str.parse::<i32>().unwrap();
            
            if age > 60 {
                count += 1;
            }
        }
        
        count

    }
}
