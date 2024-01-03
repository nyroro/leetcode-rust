
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut map = std::collections::HashMap::new();
        let mut result = Vec::new();

        for num in nums2 {
            while !stack.is_empty() && num > *stack.last().unwrap() {
                map.insert(stack.pop().unwrap(), num);
            }
            stack.push(num);
        }

        for num in nums1 {
            result.push(*map.get(&num).unwrap_or(&-1));
        }

        result

    }
}
