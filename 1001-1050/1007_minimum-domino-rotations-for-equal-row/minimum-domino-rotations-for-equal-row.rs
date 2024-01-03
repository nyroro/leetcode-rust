
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let n = tops.len();
        let mut min_swaps = n as i32 + 1;

        for num in 1..=6 {
            let mut top_swaps = 0;
            let mut bottom_swaps = 0;
            let mut possible = true;

            for i in 0..n {
                if tops[i] != num && bottoms[i] != num {
                    possible = false;
                    break;
                } else if tops[i] != num {
                    top_swaps += 1;
                } else if bottoms[i] != num {
                    bottom_swaps += 1;
                }
            }

            if possible {
                min_swaps = min_swaps.min(top_swaps).min(bottom_swaps);
            }
        }

        if min_swaps > n as i32 {
            -1

        } else {
            min_swaps

        }
    }
}
