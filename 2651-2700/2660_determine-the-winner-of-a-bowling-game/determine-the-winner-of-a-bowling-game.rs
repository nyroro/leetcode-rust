


impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let mut score1 = 0;
        let mut score2 = 0;
        let mut last_turn_strike1 = false;
        let mut last_turn_strike2 = false;

        for i in 0..player1.len() {
            let turn_score1 = if last_turn_strike1 {
                2 * player1[i]
            } else {
                player1[i]
            };
            let turn_score2 = if last_turn_strike2 {
                2 * player2[i]
            } else {
                player2[i]
            };
            score1 += turn_score1;
            score2 += turn_score2;

            last_turn_strike1 = player1[i] == 10 || (i > 0 && player1[i-1] == 10);
            last_turn_strike2 = player2[i] == 10 || (i > 0 && player2[i-1] == 10);
        }

        if score1 > score2 {
            1

        } else if score2 > score1 {
            2

        } else {
            0

        }
    }
}
