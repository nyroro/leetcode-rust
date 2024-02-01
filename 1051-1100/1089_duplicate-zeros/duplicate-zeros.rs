
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut zeros_to_duplicate = 0;
        let mut i = 0;

        // 计算需要复制的零元素的个数

        while i + zeros_to_duplicate < arr.len() {
            if arr[i] == 0 {
                zeros_to_duplicate += 1;
            }
            i += 1;
        }

        // 从数组末尾开始向前遍历，将复制的零元素插入到相应的位置上

        i = (arr.len() - 1) as i32;
        let mut j = (arr.len() + zeros_to_duplicate - 1) as i32;
        while zeros_to_duplicate > 0 {
            if j < arr.len() as i32 {
                arr[j as usize] = arr[i as usize];
            }
            j -= 1;
            if arr[i as usize] == 0 {
                zeros_to_duplicate -= 1;
                if j < arr.len() as i32 {
                    arr[j as usize] = 0;
                }
            }
            i -= 1;
        }
    }
}
