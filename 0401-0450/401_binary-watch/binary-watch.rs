
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        fn count_bits(num: i32) -> i32 {
            let mut count = 0;
            let mut n = num;
            while n > 0 {
                count += n & 1;
                n >>= 1;
            }
            count

        }
        
        fn generate_times(turned_on: i32, hours: i32, minutes: i32, result: &mut Vec<String>) {
            if count_bits(hours) + count_bits(minutes) == turned_on {
                let time = format!("{}:{:02}", hours, minutes);
                result.push(time);
            }
        }
        
        let mut result = Vec::new();
        
        for hour in 0..12 {
            for minute in 0..60 {
                generate_times(turned_on, hour, minute, &mut result);
            }
        }
        
        result

    }
}
