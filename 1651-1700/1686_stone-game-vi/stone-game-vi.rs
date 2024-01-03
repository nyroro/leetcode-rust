
impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let mut alice_points = 0;
        let mut bob_points = 0;
        
        let n = alice_values.len();
        
        let mut combined_values: Vec<(i32, i32, i32)> = Vec::new();
        
        for i in 0..n {
            combined_values.push((alice_values[i] + bob_values[i], alice_values[i], bob_values[i]));
        }
        
        combined_values.sort_by(|a, b| b.0.cmp(&a.0));
        
        for i in 0..n {
            if i % 2 == 0 {
                alice_points += combined_values[i].1;
            } else {
                bob_points += combined_values[i].2;
            }
        }
        
        if alice_points > bob_points {
            return 1;
        } else if alice_points < bob_points {
            return -1;
        } else {
            return 0;
        }
    }
}
