


impl Solution {
    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        let n = alice_arrows.len();
        let mut max_score = 0;
        let mut ret = vec![0; n];
        
        for t in 2..(1 << n) {
            let mut arrows = vec![0; n];
            let mut total_arrows = 0;
            let mut score = 0;
            
            for i in 0..n {
                if t & (1 << i) != 0 {
                    total_arrows += alice_arrows[i] + 1;
                    score += i as i32;
                    arrows[i] = alice_arrows[i] + 1;
                }
            }
            
            if total_arrows <= num_arrows && score > max_score {
                max_score = score;
                ret = arrows;
                ret[0] = num_arrows - ret.iter().sum::<i32>();
            }
        }
        
        ret

    }
}
