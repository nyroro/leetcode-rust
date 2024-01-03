
use std::collections::HashMap;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; rains.len()];
        let mut last_rain = HashMap::new();
        let mut dry_lakes = Vec::new();
        
        for i in 0..rains.len() {
            let lake = rains[i];
            
            if lake > 0 {
                if last_rain.contains_key(&lake) {
                    let last_day = last_rain[&lake];
                    let mut found = false;
                    
                    for j in 0..dry_lakes.len() {
                        if dry_lakes[j] > last_day {
                            ans[dry_lakes[j]] = lake;
                            dry_lakes.remove(j);
                            found = true;
                            break;
                        }
                    }
                    
                    if !found {
                        return vec![];
                    }
                }
                
                last_rain.insert(lake, i);
            } else {
                dry_lakes.push(i);
            }
        }
        
        for i in 0..dry_lakes.len() {
            ans[dry_lakes[i]] = 1;
        }
        
        ans

    }
}
