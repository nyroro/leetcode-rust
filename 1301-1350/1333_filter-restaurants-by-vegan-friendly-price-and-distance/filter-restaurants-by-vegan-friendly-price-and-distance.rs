
impl Solution {
    pub fn filter_restaurants(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
        // Step 1: Create an empty array to store filtered restaurants

        let mut filtered_restaurants: Vec<Vec<i32>> = Vec::new();
        
        // Step 2: Filter restaurants based on given conditions

        for restaurant in restaurants {
            let id = restaurant[0];
            let rating = restaurant[1];
            let vegan_friendly_i = restaurant[2];
            let price = restaurant[3];
            let distance = restaurant[4];
            
            if (vegan_friendly == 1 && vegan_friendly_i == 1) || vegan_friendly == 0 {
                if price <= max_price && distance <= max_distance {
                    filtered_restaurants.push(restaurant);
                }
            }
        }
        
        // Step 3: Sort filtered restaurants by rating (high to low) and then by ID (high to low)
        filtered_restaurants.sort_by(|a, b| {
            if a[1] == b[1] {
                b[0].cmp(&a[0])
            } else {
                b[1].cmp(&a[1])
            }
        });
        
        // Step 4: Extract restaurant IDs from sorted restaurants

        let result: Vec<i32> = filtered_restaurants.iter().map(|restaurant| restaurant[0]).collect();
        
        // Step 5: Return the result

        result

    }
}
