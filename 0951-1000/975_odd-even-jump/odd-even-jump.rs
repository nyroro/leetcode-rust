
impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut odd = vec![false; n];
        let mut even = vec![false; n];
        odd[n - 1] = true;
        even[n - 1] = true;
        let mut result = 1;

        let mut sorted_indices = (0..n).collect::<Vec<usize>>();
        sorted_indices.sort_by_key(|&i| arr[i]);

        let mut odd_next = Solution::next_jump_indices(&sorted_indices);
        sorted_indices.sort_by_key(|&i| -arr[i]);
        let mut even_next = Solution::next_jump_indices(&sorted_indices);

        for i in (0..n - 1).rev() {
            if let Some(&j) = odd_next.get(&i) {
                odd[i] = even[j];
            }
            if let Some(&j) = even_next.get(&i) {
                even[i] = odd[j];
            }
            if odd[i] {
                result += 1;
            }
        }

        result

    }

    fn next_jump_indices(sorted_indices: &Vec<usize>) -> std::collections::BTreeMap<usize, usize> {
        let mut stack = Vec::new();
        let mut next = std::collections::BTreeMap::new();
        for &i in sorted_indices.iter() {
            while let Some(&j) = stack.last() {
                if j < i {
                    next.insert(j, i);
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        next

    }
}
