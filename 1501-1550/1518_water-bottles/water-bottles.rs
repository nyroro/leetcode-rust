
impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total = num_bottles;
        let mut empty = num_bottles;
        
        while empty >= num_exchange {
            let new_bottles = empty / num_exchange;
            total += new_bottles;
            empty = new_bottles + (empty % num_exchange);
        }
        
        return total;
    }
}
