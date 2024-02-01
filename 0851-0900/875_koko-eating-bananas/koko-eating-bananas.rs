
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        
        while left < right {
            let mid = left + (right - left) / 2;
            
            if can_eat_all(&piles, h, mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        
        left

    }
    
    fn can_eat_all(piles: &Vec<i32>, h: i32, k: i32) -> bool {
        let mut total_time = 0;
        
        for pile in piles {
            total_time += (*pile as f64 / k as f64).ceil() as i32;
        }
        
        total_time <= h

    }
}
