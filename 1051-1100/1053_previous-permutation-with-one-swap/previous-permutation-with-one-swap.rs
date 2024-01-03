
impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let n = arr.len();
        let mut i = (n - 2) as i32;
        while i >= 0 && arr[i as usize] <= arr[(i + 1) as usize] {
            i -= 1;
        }
        if i == -1 {
            return arr;
        }
        let mut j = n - 1;
        while arr[j] >= arr[i as usize] || (j > 0 && arr[j] == arr[j - 1]) {
            j -= 1;
        }
        arr.swap(i as usize, j);
        arr

    }
}
