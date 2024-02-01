
impl Solution {
    pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
        let n = cars.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut result: Vec<f64> = vec![-1.0; n];

        for i in (0..n).rev() {
            let (position_i, speed_i) = (cars[i][0], cars[i][1]);

            while !stack.is_empty() {
                let j = *stack.last().unwrap();
                let (position_j, speed_j) = (cars[j][0], cars[j][1]);

                if speed_i <= speed_j || (position_j - position_i) as f64 / (speed_i - speed_j) >= result[j] && result[j] != -1.0 {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(j) = stack.last() {
                let (position_j, speed_j) = (cars[*j][0], cars[*j][1]);
                result[i] = (position_j - position_i) as f64 / (speed_i - speed_j) as f64;
            }

            stack.push(i);
        }

        result

    }
}
