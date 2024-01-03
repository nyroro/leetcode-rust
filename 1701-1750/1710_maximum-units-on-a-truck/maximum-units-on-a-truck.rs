
impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        // 按照单位数量降序排序箱子

        let mut sorted_boxes = box_types.clone();
        sorted_boxes.sort_by(|a, b| b[1].cmp(&a[1]));

        let mut total_units = 0;
        let mut remaining_size = truck_size;

        for box_type in sorted_boxes {
            let num_boxes = box_type[0];
            let units_per_box = box_type[1];

            // 计算当前箱子能放入卡车的数量

            let num_boxes_to_put = num_boxes.min(remaining_size);

            // 更新剩余的卡车容量和总单位数量

            remaining_size -= num_boxes_to_put;
            total_units += num_boxes_to_put * units_per_box;

            // 如果卡车已经装满，退出循环

            if remaining_size == 0 {
                break;
            }
        }

        total_units

    }
}
