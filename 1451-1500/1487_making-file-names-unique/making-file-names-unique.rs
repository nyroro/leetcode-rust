
impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut used_names: HashMap<String, usize> = HashMap::new();
        let mut result: Vec<String> = Vec::new();

        for name in names {
            if !used_names.contains_key(&name) {
                used_names.insert(name.clone(), 1);
                result.push(name);
            } else {
                let mut k = used_names[&name];
                let mut new_name = format!("{}({})", name, k);

                while used_names.contains_key(&new_name) {
                    k += 1;
                    new_name = format!("{}({})", name, k);
                }

                used_names.insert(new_name.clone(), 1);
                result.push(new_name);
                used_names.insert(name, k + 1);
            }
        }

        result

    }
}
