
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut pairs: Vec<i32> = vec![0; spells.len()]; // 步骤1


        let mut potions = potions;
        potions.sort(); // 步骤1


        for i in 0..spells.len() {
            let mut left = 0;
            let mut right = potions.len() as i32 - 1;
            while left <= right {
                let mid = left + (right - left) / 2;
                if (spells[i] as i64) * (potions[mid as usize] as i64) < success {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            pairs[i] = potions.len() as i32 - left; // 步骤2

        }

        pairs // 步骤3

    }
}
