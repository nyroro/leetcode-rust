
use std::collections::HashSet;



impl Solution {
    pub fn longest_common_subpath(n: i32, paths: Vec<Vec<i32>>) -> i32 {
        let mut l = 0;
        let mut r = paths[0].len();

        while l < r {
            let m = l + (r - l + 1) / 2;
            if Solution::check_common_subpath(&paths, m) {
                l = m;
            } else {
                r = m - 1;
            }
        }

        l as i32  // Convert l to i32 before returning

    }

    fn check_common_subpath(paths: &Vec<Vec<i32>>, m: usize) -> bool {
        let mut hash_sets: Vec<HashSet<u64>> = Vec::new();

        for path in paths {
            hash_sets.push(Solution::rabin_karp(&path, m));
        }

        for &subpath_hash in &hash_sets[0] {
            if hash_sets.iter().all(|hash_set| hash_set.contains(&subpath_hash)) {
                return true;
            }
        }

        false

    }

    fn rabin_karp(path: &Vec<i32>, m: usize) -> HashSet<u64> {
        let mut hashes: HashSet<u64> = HashSet::new();
        let mut max_power = 1;
        let mut hash = 0;

        for (i, &val) in path.iter().enumerate() {
            hash = (hash * 165131 + val as u64) % 8417508174513;
            if i >= m {
                hash = (hash + 8417508174513 - (path[i - m] as u64 * max_power % 8417508174513)) % 8417508174513;
            } else {
                max_power = max_power * 165131 % 8417508174513;
            }
            if i >= m - 1 {
                hashes.insert(hash);
            }
        }

        hashes

    }
}
