
impl Solution {
    pub fn rotate_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut result = vec![vec![0; n]; m];

        for layer in 0..(std::cmp::min(m, n) / 2) {
            let mut layer_elements = Vec::new();
            for j in layer..(n - layer) {
                layer_elements.push(grid[layer][j]);
            }
            for i in (layer + 1)..(m - layer) {
                layer_elements.push(grid[i][n - layer - 1]);
            }
            for j in ((layer)..(n - layer - 1)).rev() {
                layer_elements.push(grid[m - layer - 1][j]);
            }
            for i in ((layer + 1)..(m - layer - 1)).rev() {
                layer_elements.push(grid[i][layer]);
            }
            let rotation = k as usize % layer_elements.len();
            let rotated_layer = [&layer_elements[rotation..], &layer_elements[..rotation]].concat();
            let mut index = 0;
            for j in layer..(n - layer) {
                result[layer][j] = rotated_layer[index];
                index += 1;
            }
            for i in (layer + 1)..(m - layer) {
                result[i][n - layer - 1] = rotated_layer[index];
                index += 1;
            }
            for j in ((layer)..(n - layer - 1)).rev() {
                result[m - layer - 1][j] = rotated_layer[index];
                index += 1;
            }
            for i in ((layer + 1)..(m - layer - 1)).rev() {
                result[i][layer] = rotated_layer[index];
                index += 1;
            }
        }
        result

    }
}
