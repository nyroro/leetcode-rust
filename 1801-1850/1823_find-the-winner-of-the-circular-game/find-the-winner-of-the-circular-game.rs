
impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut friends = vec![0; n as usize];
        let mut pointer = 0;

        for i in 1..=n {
            friends[(i - 1) as usize] = i;
        }

        while friends.len() > 1 {
            pointer = (pointer + k - 1) % friends.len();
            friends.remove(pointer);
        }

        friends[0]
    }
}
