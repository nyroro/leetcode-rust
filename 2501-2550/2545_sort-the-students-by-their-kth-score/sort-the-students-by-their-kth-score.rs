
impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut score_with_index: Vec<(i32, Vec<i32>)> = score.iter().cloned().enumerate().map(|(i, s)| (s[k as usize], s)).collect();
        score_with_index.sort_by(|a, b| b.0.cmp(&a.0));
        score_with_index.into_iter().map(|(_, s)| s).collect()
    }
}
