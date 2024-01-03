
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut content_map: HashMap<String, Vec<String>> = HashMap::new();

        for path in paths {
            let parts: Vec<&str> = path.split_whitespace().collect();
            let directory = parts[0];

            for i in 1..parts.len() {
                let file_info = parts[i];
                let file_parts: Vec<&str> = file_info.split('(').collect();
                let file_name = file_parts[0];
                let content = file_parts[1].trim_end_matches(')');

                let file_path = format!("{}/{}", directory, file_name);

                let files = content_map.entry(content.to_string()).or_insert(Vec::new());
                files.push(file_path);
            }
        }

        let mut result: Vec<Vec<String>> = Vec::new();

        for (_, files) in content_map {
            if files.len() > 1 {
                result.push(files);
            }
        }

        result

    }
}
