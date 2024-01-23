


impl Solution {
    pub fn watering_plants(plants: Vec<i32>, capacity: i32) -> i32 {
        let mut now = -1;
        let mut c = capacity;
        let mut ret = 0;
        
        for (i, &t) in plants.iter().enumerate() {
            if c >= t {
                c -= t;
                ret += 1;
            } else {
                c = capacity - t;
                ret += i as i32 + 1 + i as i32;
            }
        }
        
        ret

    }
}
