
impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut arr = arr;

        for i in (1..arr.len()).rev() {
            let max_index = Solution::find_max_index(&arr, i);
            if max_index != i {
                Solution::flip(&mut arr, max_index);
                Solution::flip(&mut arr, i);
                result.push((max_index + 1) as i32);
                result.push((i + 1) as i32);
            }
        }

        result

    }

    fn find_max_index(arr: &Vec<i32>, end: usize) -> usize {
        let mut max_index = 0;
        for i in 1..=end {
            if arr[i] > arr[max_index] {
                max_index = i;
            }
        }
        max_index

    }

    fn flip(arr: &mut Vec<i32>, k: usize) {
        let mut i = 0;
        let mut j = k;
        while i < j {
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
            i += 1;
            j -= 1;
        }
    }
}
