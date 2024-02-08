
impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        asteroids.sort(); // Sort the asteroid array


        let mut weight: i64 = mass as i64; // Convert mass to i64 for larger values


        for asteroid in asteroids {
            if weight >= asteroid as i64 {
                weight += asteroid as i64;
            } else {
                return false;
            }
        }

        true

    }
}
