
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in 0..points.len() {
            let mut distances = std::collections::HashMap::new();
            for j in 0..points.len() {
                if i != j {
                    let distance = Self::calculate_distance(&points[i], &points[j]);
                    *distances.entry(distance).or_insert(0) += 1;
                }
            }
            for (_, freq) in distances {
                count += freq * (freq - 1);
            }
        }
        count

    }
    
    fn calculate_distance(p1: &[i32], p2: &[i32]) -> i32 {
        let dx = p1[0] - p2[0];
        let dy = p1[1] - p2[1];
        dx * dx + dy * dy

    }
}
