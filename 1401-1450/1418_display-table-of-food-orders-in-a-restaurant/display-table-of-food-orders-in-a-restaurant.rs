
use std::collections::HashMap;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        // 统计每个桌号点的食物数量

        let mut table_map: HashMap<i32, HashMap<String, i32>> = HashMap::new();
        let mut food_set: std::collections::HashSet<String> = std::collections::HashSet::new();
        
        for order in &orders {
            let table_number = order[1].parse::<i32>().unwrap();
            let food_item = order[2].clone();
            
            let food_map = table_map.entry(table_number).or_insert(HashMap::new());
            *food_map.entry(food_item.clone()).or_insert(0) += 1;
            
            food_set.insert(food_item);
        }
        
        // 构建输出结果

        let mut result: Vec<Vec<String>> = Vec::new();
        
        // 添加表头

        let mut header: Vec<String> = vec!["Table".to_string()];
        let mut food_list: Vec<String> = food_set.into_iter().collect();
        food_list.sort();
        header.extend(food_list.clone());
        result.push(header);
        
        // 按桌号升序遍历HashMap，将每个桌号和对应的食物数量添加到结果中

        let mut table_list: Vec<i32> = table_map.keys().cloned().collect();
        table_list.sort();
        
        for table_number in table_list {
            let mut row: Vec<String> = vec![table_number.to_string()];
            let food_map = table_map.get(&table_number).unwrap();
            
            for food_item in &food_list {
                let count = food_map.get(food_item).unwrap_or(&0);
                row.push(count.to_string());
            }
            
            result.push(row);
        }
        
        result

    }
}
