
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<u32> = version1

            .split('.')
            .map(|s| s.parse().unwrap())
            .collect();
        let v2: Vec<u32> = version2

            .split('.')
            .map(|s| s.parse().unwrap())
            .collect();
        
        let len = std::cmp::max(v1.len(), v2.len());
        
        for i in 0..len {
            let num1 = v1.get(i).unwrap_or(&0);
            let num2 = v2.get(i).unwrap_or(&0);
            
            if num1 > num2 {
                return 1;
            } else if num1 < num2 {
                return -1;
            }
        }
        
        0

    }
}
