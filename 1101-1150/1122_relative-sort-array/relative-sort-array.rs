
impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let mut index_map: HashMap<i32, usize> = HashMap::new();
        for (i, &num) in arr2.iter().enumerate() {
            index_map.insert(num, i);
        }

        let mut arr1_clone = arr1.clone();
        arr1_clone.sort_by_key(|&num| index_map.get(&num).unwrap_or(&(std::usize::MAX)));

        let (mut present, mut absent): (Vec<i32>, Vec<i32>) = arr1_clone.into_iter().partition(|&num| index_map.contains_key(&num));

        absent.sort_unstable();
        present.append(&mut absent);
        present

    }
}
