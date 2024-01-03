


impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut alice_moves = 0;
        let mut bob_moves = 0;
        let colors: Vec<char> = colors.chars().collect();
        
        for i in 1..colors.len() - 1 {
            if colors[i - 1] == 'A' && colors[i] == 'A' && colors[i + 1] == 'A' {
                alice_moves += 1;
            } else if colors[i - 1] == 'B' && colors[i] == 'B' && colors[i + 1] == 'B' {
                bob_moves += 1;
            }
        }
        
        alice_moves > bob_moves

    }
}
