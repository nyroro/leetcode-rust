


impl Solution {
    pub fn min_number_of_hours(initial_energy: i32, initial_experience: i32, energy: Vec<i32>, experience: Vec<i32>) -> i32 {
        let mut current_energy = initial_energy;
        let mut current_experience = initial_experience;
        let n = energy.len();
        let mut needed = 0;
        let mut delta = 0;

        for i in 0..n {
            if current_energy <= energy[i] {
                delta = energy[i] + 1 - current_energy;
                current_energy += delta;
                needed += delta;
            }
            current_energy -= energy[i];

            if current_experience <= experience[i] {
                delta = experience[i] + 1 - current_experience;
                current_experience += delta;
                needed += delta;
            }
            current_experience += experience[i];
        }

        needed

    }
}
