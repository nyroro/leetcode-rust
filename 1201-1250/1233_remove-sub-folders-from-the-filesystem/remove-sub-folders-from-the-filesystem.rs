
use std::collections::HashSet;

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        let mut folders = HashSet::new();

        // 将所有文件夹路径添加到哈希集合中

        for f in folder {
            folders.insert(f);
        }

        // 遍历每个文件夹路径，检查是否存在子文件夹

        for f in folders {
            let mut is_subfolder = false;
            let mut path = String::new();

            // 拆分文件夹路径为每个子文件夹

            for folder in f.split('/').skip(1) {
                path.push('/');
                path.push_str(folder);

                // 如果子文件夹存在于哈希集合中，则该文件夹是子文件夹

                if folders.contains(&path) {
                    is_subfolder = true;
                    break;
                }
            }

            // 如果不是子文件夹，则将文件夹路径添加到结果列表中

            if !is_subfolder {
                result.push(f);
            }
        }

        result

    }
}
