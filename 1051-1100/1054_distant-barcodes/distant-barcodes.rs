
use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn rearrange_barcodes(barcodes: Vec<i32>) -> Vec<i32> {
        // 统计每个条形码的出现次数

        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for barcode in barcodes.iter() {
            *count_map.entry(*barcode).or_insert(0) += 1;
        }
        
        // 使用最大堆按照出现次数排序

        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for (barcode, count) in count_map {
            heap.push((count, barcode));
        }
        
        // 创建结果数组

        let mut result: Vec<i32> = vec![0; barcodes.len()];
        
        // 从结果数组的索引0开始，每隔一个位置插入一个条形码

        let mut index = 0;
        while let Some((count, barcode)) = heap.pop() {
            for _ in 0..count {
                result[index] = barcode;
                index += 2;
                if index >= barcodes.len() {
                    index = 1;
                }
            }
        }
        
        result

    }
}
