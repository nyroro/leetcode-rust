
use std::collections::HashSet;

impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let favorite_companies_set: Vec<HashSet<String>> = favorite_companies

            .iter()
            .map(|companies| companies.iter().cloned().collect())
            .collect();

        let mut result = Vec::new();

        for (i, person1) in favorite_companies_set.iter().enumerate() {
            let mut is_subset_of_others = false;
            for (j, person2) in favorite_companies_set.iter().enumerate() {
                if i != j && person2.is_superset(person1) {
                    is_subset_of_others = true;
                    break;
                }
            }
            if !is_subset_of_others {
                result.push(i as i32);
            }
        }

        result

    }
}
