
impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        let mut label = label;
        let mut level = 0;
        while (1 << level) <= label {
            level += 1;
        }
        let mut result = vec![0; level as usize];
        while label > 0 {
            result[(level - 1) as usize] = label;
            label = (1 << level) - 1 - label + (1 << (level - 1));
            label /= 2;
            level -= 1;
        }
        result

    }
}
