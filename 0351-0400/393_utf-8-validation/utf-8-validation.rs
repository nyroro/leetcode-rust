
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut count = 0;
        
        for num in data {
            if count == 0 {
                if num >> 5 == 0b110 {
                    count = 1;
                } else if num >> 4 == 0b1110 {
                    count = 2;
                } else if num >> 3 == 0b11110 {
                    count = 3;
                } else if num >> 7 != 0 {
                    return false;
                }
            } else {
                if num >> 6 != 0b10 {
                    return false;
                }
                count -= 1;
            }
        }
        
        count == 0

    }
}
