
use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut reserved_map: HashMap<i32, i32> = HashMap::new();
        
        // Step 2: Populate the reserved_map

        for seat in reserved_seats {
            let row = seat[0];
            let seat_num = seat[1];
            let entry = reserved_map.entry(row).or_insert(0);
            *entry |= 1 << (seat_num - 1);
        }
        
        let mut max_groups = 2 * n;
        
        // Step 3: Calculate the number of four-person groups for each row

        for (_, &mask) in reserved_map.iter() {
            let mut groups = 0;
            if (mask & 0b0111111110) == 0 {
                groups += 2;
            } else if (mask & 0b0000011110) == 0 || (mask & 0b0111100000) == 0 || (mask & 0b0001111000) == 0 {
                groups += 1;
            }
            max_groups += groups - 2;
        }
        
        // Step 4: Return the total maximum number of groups

        max_groups

    }
}
