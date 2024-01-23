
impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut a = vec![0, 0, 0];
        for &s in stones.iter() {
            a[(s % 3) as usize] += 1;
        }
        
        let flip = a[0] % 2 != 0;
        
        fn gao(x: i32, flip: bool) -> bool {
            if x < 3 {
                false

            } else {
                flip

            }
        }
        
        if a[1] == 0 {
            return gao(a[2], flip);
        }
        
        if a[2] == 0 {
            return gao(a[1], flip);
        }
        
        if (a[1] - a[2]).abs() >= 3 {
            true

        } else {
            !flip

        }
    }
}
