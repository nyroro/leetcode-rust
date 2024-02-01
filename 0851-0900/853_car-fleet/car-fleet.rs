
// Step 1: Define a struct to represent each car

struct Car {
    position: i32,
    time_to_target: f64,
}

// Step 2: Implement the car_fleet function

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = position.len();
        if n == 0 {
            return 0;
        }
        
        // Create a vector of Car structs

        let mut cars: Vec<Car> = Vec::new();
        for i in 0..n {
            cars.push(Car {
                position: position[i],
                time_to_target: (target - position[i]) as f64 / speed[i] as f64,
            });
        }
        
        // Sort the cars based on their positions

        cars.sort_by(|a, b| a.position.cmp(&b.position));
        
        let mut fleet_count = 0;
        let mut t = n;
        while t > 1 {
            t -= 1;
            if cars[t].time_to_target < cars[t - 1].time_to_target {
                fleet_count += 1;
            } else {
                cars[t - 1] = cars[t];
            }
        }
        
        fleet_count + 1

    }
}
