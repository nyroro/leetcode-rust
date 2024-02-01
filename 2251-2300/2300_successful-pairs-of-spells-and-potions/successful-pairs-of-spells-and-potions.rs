
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut pairs: Vec<i32> = vec![0; spells.len()]; // 步骤1


        let mut spells = spells;
        let mut potions = potions;
        spells.sort(); // 步骤2

        potions.sort();

        let mut j = 0;
        for i in 0..spells.len() {
            let mut count = 0; // 步骤3

            while j < potions.len() && spells[i] as i64 * potions[j] as i64 < success {
                j += 1;
            }
            count = (potions.len() - j) as i32; // 步骤5

            pairs[i] = count; // 步骤7

        }

        pairs // 步骤8

    }
}
