
impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks;
        tasks.sort_by(|a, b| (b[1] - b[0]).cmp(&(a[1] - a[0])));
        let mut min_energy = 0;
        let mut current_energy = 0;
        for task in tasks {
            if current_energy < task[1] {
                min_energy += task[1] - current_energy;
                current_energy = task[1];
            }
            current_energy -= task[0];
        }
        min_energy

    }
}
