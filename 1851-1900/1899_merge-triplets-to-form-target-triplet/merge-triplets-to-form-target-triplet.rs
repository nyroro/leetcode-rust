
impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let mut updated_target = vec![0; 3];
        let mut found = false;
        
        for triplet in triplets {
            if triplet[0] <= target[0] && triplet[1] <= target[1] && triplet[2] <= target[2] {
                updated_target[0] = updated_target[0].max(triplet[0]);
                updated_target[1] = updated_target[1].max(triplet[1]);
                updated_target[2] = updated_target[2].max(triplet[2]);
                if updated_target == target {
                    found = true;
                    break;
                }
            }
        }
        
        found

    }
}
