
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut res = 0;
        let mut last_row_laser_num = 0;
        let n = bank[0].len();

        for i in 0..bank.len() {
            let mut laser_count = 0;
            for j in 0..n {
                if bank[i].chars().nth(j).unwrap() == '1' {
                    laser_count += 1;
                }
            }
            if laser_count != 0 {
                res += last_row_laser_num * laser_count;
                last_row_laser_num = laser_count;
            }
        }
        return res;
    }
}
