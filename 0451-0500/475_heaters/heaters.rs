
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        let mut heaters = heaters;
        houses.sort();
        heaters.sort();
        
        let mut radius = 0;
        
        for house in houses {
            let mut left = 0;
            let mut right = heaters.len();
            
            while left < right {
                let mid = left + (right - left) / 2;
                if heaters[mid] < house {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            
            let closest_heater_dist = if right == 0 {
                heaters[right] - house

            } else if right == heaters.len() {
                house - heaters[right - 1]
            } else {
                i32::min(heaters[right] - house, house - heaters[right - 1])
            };
            
            radius = i32::max(radius, closest_heater_dist);
        }
        
        radius

    }
}
