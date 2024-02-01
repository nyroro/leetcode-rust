
use std::collections::HashMap;

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        // 统计每个条形码的出现次数

        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for barcode in barcodes.iter() {
            *count_map.entry(*barcode).or_insert(0) += 1;
        }
        
        // 根据出现次数进行排序

        let mut sorted_barcodes: Vec<i32> = barcodes.clone();
        sorted_barcodes.sort_by_key(|&barcode| count_map[&barcode]);
        
        // 创建结果数组

        let mut result: Vec<i32> = vec![0; barcodes.len()];
        
        // 从结果数组的索引0开始，每隔一个位置插入一个条形码

        let mut index = 0;
        for i in (0..barcodes.len()).step_by(2) {
            result[i] = sorted_barcodes[index];
            index += 1;
        }
        for i in (1..barcodes.len()).step_by(2) {
            result[i] = sorted_barcodes[index];
            index += 1;
        }
        
        result

    }
}
