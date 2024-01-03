
impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let (mut low, mut high) = (0, m - 1);
        let mut max_index = 0;
        
        while low < high {
            let mid = (high - low) / 2 + low;
            max_index = Solution::get_max_index(&mat[mid], n);
            if mat[mid][max_index] < mat[mid + 1][max_index] {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        
        max_index = Solution::get_max_index(&mat[low], n);
        vec![low as i32, max_index as i32]
    }
    
    fn get_max_index(arr: &Vec<i32>, n: usize) -> usize {
        let (mut index, mut max_num) = (0, 0);
        for i in 0..n {
            if max_num < arr[i] {
                max_num = arr[i];
                index = i;
            }
        }
        index

    }
}
