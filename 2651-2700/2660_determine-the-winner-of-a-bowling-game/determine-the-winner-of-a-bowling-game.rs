


impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let mut score1 = 0;
        let mut score2 = 0;

        for i in 0..player1.len() {
            let turn_score1 = if i > 1 && player1[i-1] == 10 && player1[i-2] == 10 {
                2 * player1[i]
            } else {
                player1[i]
            };
            let turn_score2 = if i > 1 && player2[i-1] == 10 && player2[i-2] == 10 {
                2 * player2[i]
            } else {
                player2[i]
            };
            score1 += turn_score1;
            score2 += turn_score2;
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

fn main() {
    let player1 = vec![4, 10, 7, 9];
    let player2 = vec![6, 5, 2, 3];
    println!("{}", Solution::is_winner(player1, player2)); // Output: 1


    let player1 = vec![3, 5, 7, 6];
    let player2 = vec![8, 10, 10, 2];
    println!("{}", Solution::is_winner(player1, player2)); // Output: 2


    let player1 = vec![2, 3];
    let player2 = vec![4, 1];
    println!("{}", Solution::is_winner(player1, player2)); // Output: 0

}
