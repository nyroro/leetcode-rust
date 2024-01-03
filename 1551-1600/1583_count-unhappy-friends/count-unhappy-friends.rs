
impl Solution {
    pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
        let mut unhappy_count = 0;
        let mut pair_map = vec![0; n as usize];
        
        for pair in &pairs {
            pair_map[pair[0] as usize] = pair[1];
            pair_map[pair[1] as usize] = pair[0];
        }
        
        for i in 0..n as usize {
            let first_friend = i as i32;
            let pair_friend = pair_map[i] as i32;
            let first_friend_pref = &preferences[i];
            
            for &friend in first_friend_pref {
                if friend == pair_friend {
                    break;
                }
                let friend_pref = &preferences[friend as usize];
                if friend_pref.iter().position(|&x| x == first_friend).unwrap() < friend_pref.iter().position(|&x| x == pair_map[friend as usize]).unwrap() {
                    unhappy_count += 1;
                    break;
                }
            }
        }
        
        unhappy_count

    }
}
