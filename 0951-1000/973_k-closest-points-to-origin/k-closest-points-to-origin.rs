
// Define a struct to store the coordinates and distance of each point

struct PointDistance {
    coordinates: Vec<i32>,
    distance: i32,
}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // Create a vector to store the points with their distances

        let mut point_distances: Vec<PointDistance> = Vec::new();
        
        // Calculate the distance for each point and store it in the vector

        for point in points {
            let distance = point[0].pow(2) + point[1].pow(2);
            point_distances.push(PointDistance {
                coordinates: point,
                distance: distance,
            });
        }
        
        // Sort the points based on their distances

        point_distances.sort_by(|a, b| a.distance.cmp(&b.distance));
        
        // Create a vector to store the k closest points

        let mut result: Vec<Vec<i32>> = Vec::new();
        
        // Push the coordinates of the k closest points into the result vector

        for i in 0..k {
            result.push(point_distances[i as usize].coordinates.clone());
        }
        
        // Return the k closest points

        result

    }
}
