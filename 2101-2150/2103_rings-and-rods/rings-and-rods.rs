
impl Solution {
    pub fn count_points(rings: String) -> i32 {
        // Step 1: Create a HashMap to store the count of different colored rings on each rod

        let mut rod_colors = std::collections::HashMap::new();
        
        // Step 2: Parse the rings string and update the count of different colored rings on each rod

        let mut i = 0;
        while i < rings.len() {
            let color = rings.chars().nth(i).unwrap();
            let rod = rings.chars().nth(i + 1).unwrap();
            let count = rod_colors.entry(rod).or_insert([0, 0, 0]);
            match color {
                'R' => count[0] += 1,
                'G' => count[1] += 1,
                'B' => count[2] += 1,
                _ => (),
            }
            i += 2;
        }
        
        // Step 3: Count the number of rods with all three colors

        let mut result = 0;
        for (_, count) in rod_colors {
            if count[0] > 0 && count[1] > 0 && count[2] > 0 {
                result += 1;
            }
        }
        
        result

    }
}
